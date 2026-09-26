# Antora for Zed

Zed extension that attaches the [antora-ls](../antora-ls) language server to AsciiDoc files: `xref:` / `include::` / `image:` completion, go-to-definition, hover, and broken-reference diagnostics.

It layers on top of Zed's **AsciiDoc** extension (syntax highlighting), so install that first.

## How the server is found

1. `lsp.antora-ls.binary.path` in Zed settings
2. `antora-ls` on your `PATH`
3. The latest GitHub release of `antora-ls` for your platform (downloaded automatically)

```json
{ "lsp": { "antora-ls": { "binary": { "path": "/path/to/antora-ls" } } } }
```

## Develop

```sh
rustup target add wasm32-wasip2
```

In Zed: install the **AsciiDoc** extension, run `zed: install dev extension`, and pick this folder. Open a folder containing `antora.yml` and edit a page.

## Publishing

Submit this repo to [`zed-industries/extensions`](https://github.com/zed-industries/extensions).

## License

MIT — see [LICENSE](LICENSE). The bundled language server, [antora-ls](../antora-ls), is Apache-2.0 and derived from the [AsciiDoc plugin for IntelliJ](https://github.com/asciidoctor/asciidoctor-intellij-plugin).
