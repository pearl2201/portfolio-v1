use qrcode::{render::svg, QrCode};
use serde::{Deserialize, Serialize};
use worker::*;

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

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/foo", handle_get)
        .post_async("/generate", handle_post)
        .run(req, env)
        .await
}

pub async fn handle_get(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a GET route!".to_string(),
    })
}

pub async fn handle_post(mut req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let gen = match req.json::<GenerateRequest>().await {
        Ok(value) => value,
        Err(_) => return Response::error("Bad request", 400),
    };

    let code = QrCode::new(gen.content).unwrap();
    let image = code
        .render()
        .min_dimensions(gen.size, gen.size)
        .dark_color(svg::Color(&gen.dark_color))
        .light_color(svg::Color(&gen.light_color))
        .build();

    Response::from_json(&GenericResponse {
        status: 200,
        message: image,
    })
}
