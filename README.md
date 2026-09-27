# Antora for Zed

Antora support for [Zed](https://zed.dev): completion, navigation, refactoring, diagnostics and previews for [Antora](https://antora.org) documentation sites. The work is done by the [antora-ls](https://github.com/moritosh/antora-ls) language server; this extension finds or downloads it and attaches it to AsciiDoc files.

It layers on top of Zed's **AsciiDoc** extension (syntax highlighting), so install that first.

## Features

Default macOS keys; on Linux and Windows use `ctrl` for `cmd`.

| Feature | How |
| --- | --- |
| Completion of `xref:`, `include::` and `image:` targets across modules, components and versions, and of `{attributes}` | as you type, or `ctrl-space` |
| Go to definition, landing on `#fragment` anchors | `f12` or `cmd`-click; `cmd`-hover underlines the whole target |
| Hover: target title, component version, module, path, alias | mouse over, or `cmd-k cmd-i` |
| Find references to a page, partial, example, image or attachment | `alt-shift-f12` on a reference or in the file |
| Rename a resource: moves the file and updates every reference | `f2` on a reference, then type the new path (e.g. `setup/linux`) |
| Move or rename files and folders in the project panel: references follow | drag or rename in the project panel |
| Diagnostics: broken references, nav file structure, unregistered nav files, missing `nav:` entries | inline, or `cmd-shift-m` for all |
| `page-aliases`: references to an alias resolve to the page | automatic |
| Playbooks: local content sources and site-wide attributes | automatic |
| Live site preview in the browser | `cmd-.` → **Antora: Open live site preview** |
| Markdown preview inside Zed | `cmd-.` → **Antora: Open Markdown preview**, then `cmd-k v` |

To see a referenced file in the project panel, jump to it and run `pane: reveal in project panel` (`cmd-shift-e`).

## Previews

**Live site preview** — your real site (UI bundle, xrefs, includes, navigation) in the browser, built from HEAD. Edits appear as you type without a page reload, and the browser scrolls to the block you're editing and follows you to other pages. Needs [Bun](https://bun.sh) or Node, and Antora 3 installed in the docs project.

**Markdown preview** — an approximate rendering inside Zed. Run the action once and press `cmd-k v` in the `Antora Preview.md` tab it opens; the preview then follows whichever page you work on and updates as you type. xrefs, includes, images and attributes are resolved; admonitions and sidebars show as quotes.

Details are in the [antora-ls README](https://github.com/moritosh/antora-ls#previews).

## Settings

The language server is found in this order:

1. `lsp.antora-ls.binary.path` in Zed settings
2. `antora-ls` on your `PATH` (e.g. `cargo install --path antora-ls`)
3. The latest [GitHub release](https://github.com/moritosh/antora-ls/releases) for your platform, downloaded automatically — this needs the `antora-ls` repository to be public

To choose the binary or the JavaScript runtime used by the live preview:

```json
{
  "lsp": {
    "antora-ls": {
      "binary": { "path": "/path/to/antora-ls" },
      "initialization_options": { "preview": { "runtime": "/path/to/bun" } }
    }
  }
}
```

## Develop

```sh
rustup target add wasm32-wasip2
```

In Zed: install the **AsciiDoc** extension, run `zed: install dev extension`, and pick this folder. Open a folder containing `antora.yml` and edit a page. After changing `antora-ls`, reinstall it and run `editor: restart language server`.

## Publishing

Make `antora-ls` public and publish a release, then submit this repo to [`zed-industries/extensions`](https://github.com/zed-industries/extensions).

## License

MIT — see [LICENSE](LICENSE). The language server, [antora-ls](https://github.com/moritosh/antora-ls), is Apache-2.0 and derived from the [AsciiDoc plugin for IntelliJ](https://github.com/asciidoctor/asciidoctor-intellij-plugin).
