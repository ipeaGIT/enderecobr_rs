use std::fmt::Display;

use enderecobr_rs::padronizar_logradouros;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

trait SerializadorSnapshot<T> {
    fn carregar(&self, nome_teste: &str, etapa_teste: &str) -> Vec<T>;
    fn salvar(&self, nome_teste: &str, etapa_teste: &str, valores: Vec<T>) -> Result<(), String>;
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
    ) -> Result<(), String> {
        let file_path = Path::new(BASE_PATH).join(format!("{}_{}.txt", nome_teste, etapa_teste));

        // Criar diretório base se não existir
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
        for valor in valores {
            writeln!(file, "{}", valor).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
    fn carregar(&self, nome_teste: &str, etapa_teste: &str) -> Vec<String> {
        let file_path = Path::new(BASE_PATH).join(format!("{}_{}.txt", nome_teste, etapa_teste));

        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        };

        BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .collect()
    }
}

#[derive(Default)]
struct SerializadorI32;

impl SerializadorSnapshot<i32> for SerializadorI32 {
    fn salvar(&self, nome_teste: &str, etapa_teste: &str, valores: Vec<i32>) -> Result<(), String> {
        todo!()
    }
    fn carregar(&self, nome_teste: &str, etapa_teste: &str) -> Vec<i32> {
        todo!()
    }
}

///////////////////
// Classe principal
//////////////////

trait SnapshotTester {
    fn salvar_snapshot(&self) -> Result<(), String>;
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
    fn salvar_snapshot(&self) -> Result<(), String> {
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

        let mut res = String::new();
        valores_brutos
            .iter()
            .zip(valores_snapshot.iter())
            .filter_map(|(bruto, snap)| {
                let atual = (self.processador)(bruto);
                if atual != *snap {
                    Some((bruto, snap, atual))
                } else {
                    None
                }
            })
            .for_each(|(bruto, snap, atual)| {
                res.push_str(format!("{:}\t| {:}\t| {:}\n", bruto, snap, atual).as_str());
            });

        if !res.is_empty() {
            res.insert_str(
                0,
                "Original\t| Snapshot\t| Atual\n--------------------------------------------\n",
            );
        }
        Ok(res)
    }
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

        "num extenso" => Box::new(SnapshotTesterImpl {
            nome: "numero_extenso",
            serializador_entrada: SerializadorI32,
            serializador_saida: SerializadorString,
            processador: |x: &i32| x.to_string(),
        }),

        _ => panic!("Nenhum teste encontrado"),
    }
}

////////////////

fn main() -> Result<(), String> {
    // obter_snapshot_tester_dyn("logr").salvar_snapshot();
    let res = obter_snapshot_tester_dyn("logr")
        .comparar_snapshot()
        .map_err(|x| x.to_string())?;

    println!("{}", res);
    Ok(())
}
