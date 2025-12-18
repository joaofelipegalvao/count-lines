use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: count-lines <arquivo>");
        return;
    }

    let nome_arquivo = &args[1];

    let conteudo = fs::read_to_string(nome_arquivo).unwrap();
    let linhas = conteudo.lines().count();

    println!("{} linhas", linhas);
}
