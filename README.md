# Toxic Company

WHS course. Three modules: overt narcissist, covert narcissist, corporate psychopath. Flip cards. Stamp.

Leptos → WebAssembly. Educational. Not legal advice.

## Run locally

```bash
cargo run --release --features server
# http://127.0.0.1:7420
```

Needs `wasm32-unknown-unknown` (`rustup target add wasm32-unknown-unknown`).

## GitHub Pages

Routes are hashes (`#/`, `#/map`, `#/play/...`) so Pages needs no rewrite. Assets are relative (`./pkg/...`).

1. Push this repo to GitHub.
2. **Settings → Pages → Source: GitHub Actions**.
3. Push to `main` (or run the **pages** workflow). After it finishes, the site is:

`https://<user>.github.io/<repo>/`

Local export (same files CI uploads):

```bash
make pages
# then open pages/index.html via any static server
```
