use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: count-lines [--no-empty] <arquivo>");
        process::exit(1);
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
        eprintln!("Erro: nenhum arquivo especificado");
        process::exit(1);
    }

    let conteudo = match fs::read_to_string(nome_arquivo) {
        Ok(conteudo) => conteudo,
        Err(erro) => {
            eprintln!("Erro ao ler '{}': {}", nome_arquivo, erro);
            process::exit(1);
        }
    };

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
}
