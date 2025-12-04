use std::fmt::Display;
use std::time::Instant;

use clap::Parser;
use enderecobr_rs::{
    padronizar_bairros, padronizar_complementos, padronizar_estados_para_nome,
    padronizar_logradouros, padronizar_municipios, padronizar_numeros,
};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use tabled::settings::{Style, Width};
use tabled::{Table, Tabled};

trait SerializadorSnapshot<T> {
    fn carregar(
        &self,
        base_path: &str,
        nome_teste: &str,
        etapa_teste: &str,
    ) -> Result<Vec<T>, String>;

    fn salvar(
        &self,
        base_path: &str,
        nome_teste: &str,
        etapa_teste: &str,
        valores: Vec<T>,
    ) -> Result<String, String>;
}

//////////////

#[derive(Default)]
struct SerializadorString;

impl SerializadorSnapshot<String> for SerializadorString {
    fn salvar(
        &self,
        base_path: &str,
        nome_teste: &str,
        etapa_teste: &str,
        valores: Vec<String>,
    ) -> Result<String, String> {
        let file_path = Path::new(base_path).join(format!("{}_{}.csv", nome_teste, etapa_teste));

        let mut file = File::create(&file_path).map_err(|e| {
            format!(
                "Erro ao salvar o arquivo {:}: {:}",
                file_path.to_str().unwrap_or(""),
                e
            )
        })?;
        for valor in valores {
            writeln!(file, "{}", valor).map_err(|e| e.to_string())?;
        }
        Ok(file_path
            .to_str()
            .ok_or("Caminho do arquivo inválido")?
            .to_string())
    }
    fn carregar(
        &self,
        base_path: &str,
        nome_teste: &str,
        etapa_teste: &str,
    ) -> Result<Vec<String>, String> {
        let file_path = Path::new(base_path).join(format!("{}_{}.csv", nome_teste, etapa_teste));

        let file = File::open(&file_path).map_err(|e| {
            format!(
                "Erro ao abrir arquivo {:}: {:}",
                file_path.to_str().unwrap_or(""),
                e
            )
        })?;

        Ok(BufReader::new(file)
            .lines()
            .map_while(|line| line.ok())
            .collect())
    }
}

///////////////////
// Classe principal
//////////////////

trait SnapshotTester {
    fn salvar_snapshot(&self, base_path: &str) -> Result<String, String>;
    fn comparar_snapshot(&self, base_path: &str) -> Result<String, String>;
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
    fn salvar_snapshot(&self, base_path: &str) -> Result<String, String> {
        let valores_brutos = self
            .serializador_entrada
            .carregar(base_path, self.nome, "bruto")?;
        let valores_processados: Vec<U> = valores_brutos
            .iter()
            .map(|x| (self.processador)(x))
            .collect();

        self.serializador_saida
            .salvar(base_path, self.nome, "snapshot", valores_processados)
    }

    fn comparar_snapshot(&self, base_path: &str) -> Result<String, String> {
        let valores_brutos = self
            .serializador_entrada
            .carregar(base_path, self.nome, "bruto")?;
        let valores_snapshot = self
            .serializador_saida
            .carregar(base_path, self.nome, "snapshot")?;

        assert_eq!(
            valores_snapshot.len(),
            valores_brutos.len(),
            "Os arquivos com os dados brutos e de snapshot têm tamanhos diferentes."
        );

        let inicio = Instant::now();
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

        let duracao = inicio.elapsed().as_nanos();
        let tempo_por_qtd = duracao / valores_snapshot.len() as u128;

        println!(
            "Avaliação de {}: \n- Processado {} dados em {} ns ({} ns/reg => {:.0} reg/s)",
            self.nome,
            valores_snapshot.len(),
            duracao,
            tempo_por_qtd,
            (1f64 / tempo_por_qtd as f64) * 1_000_000_000f64
        );

        if !res.is_empty() {
            return Ok(Table::new(res)
                .with(Style::modern())
                .with(Width::wrap(50))
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

fn obter_snapshot_tester_dyn(nome_teste: &str) -> Box<dyn SnapshotTester> {
    match nome_teste {
        "logr" => Box::new(SnapshotTesterImpl {
            nome: "logradouro",
            serializador_entrada: SerializadorString,
            serializador_saida: SerializadorString,
            processador: |x: &String| padronizar_logradouros(x),
        }),
        "num" => Box::new(SnapshotTesterImpl {
            nome: "numero",
            serializador_entrada: SerializadorString,
            serializador_saida: SerializadorString,
            processador: |x: &String| padronizar_numeros(x),
        }),
        "comp" => Box::new(SnapshotTesterImpl {
            nome: "complemento",
            serializador_entrada: SerializadorString,
            serializador_saida: SerializadorString,
            processador: |x: &String| padronizar_complementos(x),
        }),
        "loc" => Box::new(SnapshotTesterImpl {
            nome: "localidade",
            serializador_entrada: SerializadorString,
            serializador_saida: SerializadorString,
            processador: |x: &String| padronizar_bairros(x),
        }),
        "mun" => Box::new(SnapshotTesterImpl {
            nome: "municipio",
            serializador_entrada: SerializadorString,
            serializador_saida: SerializadorString,
            processador: |x: &String| padronizar_municipios(x),
        }),
        "uf" => Box::new(SnapshotTesterImpl {
            nome: "uf",
            serializador_entrada: SerializadorString,
            serializador_saida: SerializadorString,
            processador: |x: &String| padronizar_estados_para_nome(x),
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
    /// Caminho Base
    caminho: String,

    /// Testes a serem realizados
    tipo_teste: Vec<String>,

    /// Salvar snapshot
    #[arg(short('s'), long, default_value = "false")]
    salvar: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let tipos_testes = if !args.tipo_teste.is_empty() {
        args.tipo_teste
    } else {
        ["logr", "num", "comp", "loc", "mun", "uf"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    };

    for tipo_teste in tipos_testes {
        let tester = obter_snapshot_tester_dyn(&tipo_teste);
        if args.salvar {
            println!("Salvando snapshot para {}", tipo_teste);
            let arq = tester.salvar_snapshot(&args.caminho)?;
            println!("Snapshot salvo em {}", arq);
        } else {
            println!("Comparando snapshot para {}", tipo_teste);
            let diffs = tester.comparar_snapshot(&args.caminho);
            println!("{:}", diffs?);
        }
    }

    Ok(())
}
