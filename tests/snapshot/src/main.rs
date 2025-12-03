use std::fmt::Display;

use clap::Parser;
use enderecobr_rs::padronizar_logradouros;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use tabled::settings::{Style, Width};
use tabled::{Table, Tabled};

trait SerializadorSnapshot<T> {
    fn carregar(&self, nome_teste: &str, etapa_teste: &str) -> Vec<T>;
    fn salvar(
        &self,
        nome_teste: &str,
        etapa_teste: &str,
        valores: Vec<T>,
    ) -> Result<String, String>;
}

//////////////

const BASE_PATH: &str = "tests/snapshot/data";

#[derive(Default)]
struct SerializadorString;

impl SerializadorSnapshot<String> for SerializadorString {
    fn salvar(
        &self,
        nome_teste: &str,
        etapa_teste: &str,
        valores: Vec<String>,
    ) -> Result<String, String> {
        let file_path = Path::new(BASE_PATH).join(format!("{}_{}.txt", nome_teste, etapa_teste));

        // Criar diretório base se não existir
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
        for valor in valores {
            writeln!(file, "{}", valor).map_err(|e| e.to_string())?;
        }
        Ok(file_path
            .to_str()
            .ok_or("Caminho do arquivo inválido")?
            .to_string())
    }
    fn carregar(&self, nome_teste: &str, etapa_teste: &str) -> Vec<String> {
        let file_path = Path::new(BASE_PATH).join(format!("{}_{}.txt", nome_teste, etapa_teste));

        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        };

        BufReader::new(file)
            .lines()
            .map_while(|line| line.ok())
            .collect()
    }
}

///////////////////
// Classe principal
//////////////////

trait SnapshotTester {
    fn salvar_snapshot(&self) -> Result<String, String>;
    fn comparar_snapshot(&self) -> Result<String, String>;
}

struct SnapshotTesterImpl<T, U, I, O>
where
    I: SerializadorSnapshot<T>,
    O: SerializadorSnapshot<U>,
    U: PartialEq,
{
    nome: &'static str,
    serializador_entrada: I,
    serializador_saida: O,
    processador: fn(&T) -> U,
}

impl<T, U, I, O> SnapshotTester for SnapshotTesterImpl<T, U, I, O>
where
    I: SerializadorSnapshot<T>,
    O: SerializadorSnapshot<U>,
    T: Display,
    U: PartialEq + Display,
{
    fn salvar_snapshot(&self) -> Result<String, String> {
        let valores_brutos = self.serializador_entrada.carregar(self.nome, "bruto");
        let valores_processados: Vec<U> = valores_brutos
            .iter()
            .map(|x| (self.processador)(x))
            .collect();

        self.serializador_saida
            .salvar(self.nome, "snapshot", valores_processados)
    }

    fn comparar_snapshot(&self) -> Result<String, String> {
        let valores_brutos = self.serializador_entrada.carregar(self.nome, "bruto");
        let valores_snapshot = self.serializador_saida.carregar(self.nome, "snapshot");

        let res: Vec<_> = valores_brutos
            .iter()
            .zip(valores_snapshot.iter())
            .filter_map(|(bruto, snap)| {
                let atual = (self.processador)(bruto);
                if atual != *snap {
                    Some(Diff {
                        original: bruto.to_string(),
                        snapshot: snap.to_string(),
                        atual: atual.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        if !res.is_empty() {
            return Ok(Table::new(res)
                .with(Style::modern())
                .with(Width::wrap(100))
                .to_string());
        }

        Ok("Nenhuma mudança identificada.".to_string())
    }
}

#[derive(Tabled)]
struct Diff {
    original: String,
    snapshot: String,
    atual: String,
}

/////////////////
// Utilitários
////////////////

fn talvez_baixar_dataset_bruto() {
    todo!()
}

fn obter_snapshot_tester_dyn(nome_teste: &str) -> Box<dyn SnapshotTester> {
    match nome_teste {
        "logr" => Box::new(SnapshotTesterImpl {
            nome: "logradouro",
            serializador_entrada: SerializadorString,
            serializador_saida: SerializadorString,
            processador: |x: &String| padronizar_logradouros(x),
        }),

        _ => panic!("Nenhum teste encontrado"),
    }
}

////////////////

/// Utilitário que serve para comparar o resultado desta lib com valores
/// previamente salvos.
#[derive(Parser)]
#[clap(author, version)]
struct Args {
    /// Testes a serem realizados
    tipo_teste: Vec<String>,

    /// Salvar snapshot
    #[arg(short('s'), long, default_value = "false")]
    salvar: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    for tipo_teste in args.tipo_teste {
        let tester = obter_snapshot_tester_dyn(&tipo_teste);
        if args.salvar {
            let arq = tester.salvar_snapshot()?;
            println!("Snapshot salvo em {}", arq);
        } else {
            let diffs = tester.comparar_snapshot();
            println!("{:}", diffs?);
        }
    }

    Ok(())
}
