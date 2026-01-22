use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Erro: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Uso: count-lines [--no-empty] <arquivo>".into());
    }

    let mut ignorar_vazias = false;
    let mut nome_arquivo = "";

    for arg in &args[1..] {
        if arg == "--no-empty" {
            ignorar_vazias = true;
        } else {
            nome_arquivo = arg;
        }
    }

    if nome_arquivo.is_empty() {
        return Err("Nenhum arquivo especificado".into());
    }

    let conteudo = fs::read_to_string(nome_arquivo)?;

    let linhas = if ignorar_vazias {
        conteudo
            .lines()
            .filter(|linha| !linha.trim().is_empty())
            .count()
    } else {
        conteudo.lines().count()
    };

    let sufixo = if ignorar_vazias { " (sem vazias)" } else { "" };

    println!("{}: {} linhas{}", nome_arquivo, linhas, sufixo);
    Ok(())
}
