Contador de linhas de código - Projeto de estudo em Rust 🦀

## 📚 Evolução do projeto

Este projeto foi desenvolvido em 4 etapas, marcadas por tags:

| Versão | Descrição | Conceitos |
|--------|-----------|-----------|
| [v1] | Lê arquivo fixo, conta linhas simples | `fs::read_to_string`, `.lines()`, `.count()` |
| [v2] | Aceita arquivo via CLI | `env::args()`, `Vec<String>` |
| [v3] | Tratamento de erros idiomático | `Result<T, E>`, `match`, `eprintln!` |
| [v4] | Flag `--no-empty` para ignorar linhas vazias | `.filter()`, closures, `.trim()` |
 | [v5] | Operador `?` para propagação de erros | `Result<(), E>`, `Box<dyn Error>`, `if let`, `main()/run()`, `.into()` |
| [v6] | Suporte a múltiplos arquivos | `Vec::new()`, `.push()`, `is_empty()`, `for in`, `+=`, `.len()` |

[v1]: https://github.com/joaofelipegalvao/count-lines/releases/tag/v0.1.0
[v2]: https://github.com/joaofelipegalvao/count-lines/releases/tag/v0.2.0
[v3]: https://github.com/joaofelipegalvao/count-lines/releases/tag/v0.3.0
[v4]: https://github.com/joaofelipegalvao/count-lines/releases/tag/v0.4.0
[v5]: https://github.com/joaofelipegalvao/count-lines/releases/tag/v0.4.0
[v6]: https://github.com/joaofelipegalvao/count-lines/releases/tag/v0.6.0

## 🚀 Como usar

```bash
# Versão atual (main)
cargo run -- arquivo.txt
cargo run -- --no-empty arquivo.txt

# Ver versão específica
git checkout v0.1.0  # ou qualquer tag
```

## 💡 O que aprendi

- ✅ Manipulação de arquivos
- ✅ Argumentos de linha de comando
- ✅ Tratamento de erros idiomático
- ✅ Iteradores e filtros
- ✅ Closures
- ✅ Result<(), E> para funções que podem falhar
- ✅ Box<dyn Error> para erros genéricos
- ✅ if let para matching simplificado
- ✅ Padrão main()/run()
- ✅ Vec para coleções mutáveis
- ✅ push() para adicionar elementos
- ✅ Iteração com for in
- ✅ Operador += para acumuladores

## 🎯 Próximos passos

- [x] Refatorar com `?` operator
- [x] Múltiplos arquivos

---

**Nota:** Este é um projeto de aprendizado. Cada tag representa um passo evolutivo.
