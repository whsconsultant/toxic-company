#!/bin/sh
# Static site in ./pages for GitHub Pages (hash routes, relative pkg/).
set -eu
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
cargo build --manifest-path ui/Cargo.toml --target wasm32-unknown-unknown --release
touch build.rs
cargo build --release --features server
WEBUI="$(find target/release/build -type d -path '*/out/webui' | while read -r d; do
  if [ -f "$d/harbor_desk_ui.js" ] && [ -f "$d/harbor_desk_ui_bg.wasm" ]; then
    echo "$d"
  fi
done | tail -1)"
if [ -z "$WEBUI" ]; then
  echo "bindgen output missing" >&2
  exit 1
fi
DEST="$ROOT/pages"
rm -rf "$DEST"
mkdir -p "$DEST/pkg"
cp "$WEBUI/harbor_desk_ui.js" "$DEST/pkg/"
cp "$WEBUI/harbor_desk_ui_bg.wasm" "$DEST/pkg/"
python3 - <<'PY'
from pathlib import Path
html = """<!DOCTYPE html>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover">
<title>Toxic Company</title>
<meta name="apple-mobile-web-app-capable" content="yes">
<meta name="mobile-web-app-capable" content="yes">
<body>
<p id="boot">loading wasm…</p>
<script type="module">
  import init from "./pkg/harbor_desk_ui.js";
  try {
    await init({ module_or_path: new URL("./pkg/harbor_desk_ui_bg.wasm", import.meta.url) });
    document.getElementById("boot")?.remove();
  } catch (e) {
    document.getElementById("boot").textContent = String(e && e.message ? e.message : e);
  }
</script>
"""
Path("pages/index.html").write_text(html)
Path("pages/404.html").write_text(html)
Path("pages/.nojekyll").write_text("")
print("wrote pages/")
PY
ls -lh pages pages/pkg
