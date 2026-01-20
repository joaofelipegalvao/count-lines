Contador de linhas de código - Projeto de estudo em Rust 🦀

## 📚 Evolução do projeto

Este projeto foi desenvolvido em 4 etapas, marcadas por tags:

| Versão | Descrição | Conceitos |
|--------|-----------|-----------|
| [v1] | Lê arquivo fixo, conta linhas simples | `fs::read_to_string`, `.lines()`, `.count()` |
| [v2] | Aceita arquivo via CLI | `env::args()`, `Vec<String>` |
| [v3] | Tratamento de erros idiomático | `Result<T, E>`, `match`, `eprintln!` |
| [v4] | Flag `--no-empty` para ignorar linhas vazias | `.filter()`, closures, `.trim()` |

[v1]: https://github.com/joaofelipegalvao/count-lines/releases/tag/v0.1.0
[v2]: https://github.com/joaofelipegalvao/count-lines/releases/tag/v0.2.0
[v3]: https://github.com/joaofelipegalvao/count-lines/releases/tag/v0.3.0
[v4]: https://github.com/joaofelipegalvao/count-lines/releases/tag/v0.4.0

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
- ✅ Git tags (organização de versões)

## 🎯 Próximos passos

- [ ] Múltiplos arquivos
- [ ] Ignorar comentários
- [ ] Refatorar com `?` operator
- [ ] Testes unitários

---

**Nota:** Este é um projeto de aprendizado. Cada tag representa um passo evolutivo.
