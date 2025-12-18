use std::fs;

fn main() {
    let conteudo = fs::read_to_string("main.rs").unwrap();
    let linhas = conteudo.lines().count();
    println!("{} linhas", linhas);
}
