use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: count-lines <arquivo>");
        process::exit(1);
    }

    let nome_arquivo = &args[1];

    let conteudo = match fs::read_to_string(nome_arquivo) {
        Ok(conteudo) => conteudo,
        Err(erro) => {
            eprintln!("Erro ao ler '{}': {}", nome_arquivo, erro);
            process::exit(1);
        }
    };

    let linhas = conteudo.lines().count();

    println!("{}: {} linhas", nome_arquivo, linhas);
}
