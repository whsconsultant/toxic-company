use axum::http::header;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use std::env;
use std::net::SocketAddr;
use workplace_sim::DEFAULT_BIND;

fn bind_addr() -> SocketAddr {
    let raw = env::args()
        .nth(1)
        .filter(|a| !a.starts_with('-'))
        .unwrap_or_else(|| DEFAULT_BIND.into());
    raw.parse().unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], 7420)))
}

#[tokio::main]
async fn main() {
    if matches!(
        env::args().nth(1).as_deref(),
        Some("-h" | "--help" | "help")
    ) {
        eprintln!(
            "toxic company — workplace-pattern training (Leptos WASM)

  harbor-desk [bind]     default {DEFAULT_BIND}

Open the URL. Title → contents → flip the card → stamp.
UI is Rust compiled to wasm. No authored JavaScript or TypeScript.
"
        );
        return;
    }
    let bind = bind_addr();
    let app = Router::new()
        .route("/pkg/harbor_desk_ui.js", get(js))
        .route("/pkg/harbor_desk_ui_bg.wasm", get(wasm))
        .fallback(index);
    eprintln!("Toxic Company  http://{bind}");
    let lis = match tokio::net::TcpListener::bind(bind).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("bind {bind}: {e}");
            return;
        }
    };
    if let Err(e) = axum::serve(lis, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
    {
        eprintln!("serve: {e}");
    }
}

async fn index() -> Html<&'static str> {
    Html(INDEX)
}

async fn js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "no-store"),
        ],
        include_str!(concat!(env!("OUT_DIR"), "/webui/harbor_desk_ui.js")),
    )
        .into_response()
}

async fn wasm() -> Response {
    (
        [
            (header::CONTENT_TYPE, "application/wasm"),
            (header::CACHE_CONTROL, "no-store"),
        ],
        include_bytes!(concat!(
            env!("OUT_DIR"),
            "/webui/harbor_desk_ui_bg.wasm"
        ))
        .as_slice(),
    )
        .into_response()
}

const INDEX: &str = r#"<!DOCTYPE html>
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
"#;
