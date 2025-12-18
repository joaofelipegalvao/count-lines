Contador de linhas de código - Projeto de estudo em Rust 🦀

## 📚 Evolução do projeto

Este projeto foi desenvolvido em 4 etapas, cada uma preservada em sua própria branch:

### [v1 - Básico](../../tree/count-lines-v1)
- Lê arquivo fixo (`main.rs`)
- Conta linhas simples
- **Conceitos:** `fs::read_to_string`, `.lines()`, `.count()`

### [v2 - Argumentos](../../tree/count-lines-v2)
- Aceita nome do arquivo via CLI
- Validação básica de entrada
- **Conceitos:** `env::args()`, `Vec<String>`, indexação

### [v3 - Tratamento de erro](../../tree/count-lines-v3)
- Erros amigáveis com `Result` e `match`
- Mensagens no stderr (`eprintln!`)
- Exit codes adequados
- **Conceitos:** `Result<T, E>`, `match`, `process::exit()`

### [v4 - Flag --no-empty](../../tree/main) ← **Versão atual**
- Ignora linhas vazias com flag `--no-empty`
- Loop sobre argumentos
- **Conceitos:** `.filter()`, closures, `mut`, `.trim()`

## 🚀 Como usar
```bash
# Versão atual (main)
git checkout main
cargo run -- arquivo.txt
cargo run -- --no-empty arquivo.txt

# Voltar pra v1
git checkout count-lines-v1
cargo run

# Voltar pra v2
git checkout count-lines-v2
cargo run -- arquivo.txt
```

## 💡 O que aprendi

- ✅ Manipulação de arquivos
- ✅ Argumentos de linha de comando
- ✅ Tratamento de erros idiomático
- ✅ Iteradores e filtros
- ✅ Closures
- ✅ Git branches (organização de versões)

## 🎯 Próximos passos

- [ ] Múltiplos arquivos
- [ ] Ignorar comentários
- [ ] Refatorar com `?` operator
- [ ] Testes unitários

---

**Nota:** Este é um projeto de aprendizado. Cada branch representa um passo evolutivo.
