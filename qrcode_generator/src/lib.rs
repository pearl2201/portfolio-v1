use qrcode::{render::svg, QrCode};
use serde::{Deserialize, Serialize};
use worker::*;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GenerateRequest {
    content: String,
    size: u32,
    dark_color: String,
    light_color: String,
}
impl Default for GenerateRequest {
    fn default() -> GenerateRequest {
        GenerateRequest {
            content: "".to_string(),
            size: 200,
            dark_color: "#000".to_string(),
            light_color: "#fff".to_string(),
        }
    }
}

// Multiple calls to `init` will cause a panic as a tracing subscriber is already set.
// So we use the `start` event to initialize our tracing subscriber when the worker starts.
#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false) // Only partially supported across JavaScript runtimes
        .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/foo", handle_get)
        .post_async("/generate", handle_post)
        .options_async("/generate", handle_options)
        .run(req, env)
        .await
}

pub async fn handle_get(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a GET route!".to_string(),
    })
}

pub async fn handle_options(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let methods = ["*".to_string()];
    let cors = Cors::default().with_methods(Method::all()).with_origins(["*"]).with_allowed_headers(["*"]);
    Response::empty()?.with_cors(&cors)
}

pub async fn handle_post(mut req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let gen = match req.json::<GenerateRequest>().await {
        Ok(value) => value,
        Err(err) => {
            tracing::info!(err=?err, "err");
            return Response::error("Bad request", 400);
        }
    };

    let code = QrCode::new(gen.content).unwrap();
    let image = code
        .render()
        .min_dimensions(gen.size, gen.size)
        .dark_color(svg::Color(&gen.dark_color))
        .light_color(svg::Color(&gen.light_color))
        .build();
    let cors = Cors::default().with_methods(Method::all()).with_origins(["*"]).with_allowed_headers(["*"]);
    Response::from_json(&GenericResponse {
        status: 200,
        message: image,
    })?.with_cors(&cors)
}
