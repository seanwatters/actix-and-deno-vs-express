use actix_web::{get, App, HttpServer, Responder};

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::serde_v8;
use deno_core::v8;

static JS_FILE: &'static str = concat!(include_str!("./hello-world.js"), "main()");


fn run_js(src: &str) -> String {
    let mut js_runtime = JsRuntime::new(RuntimeOptions::default());

    let result = js_runtime.execute_script("<usage>", src).unwrap();

    let scope = &mut js_runtime.handle_scope();
    let local = v8::Local::new(scope, result);

    match serde_v8::from_v8::<serde_json::Value>(scope, local).unwrap() {
        serde_json::Value::String(val) => val,
        _ => "bad string!".to_string()
    }
}

#[get("/run")]
async fn run() -> impl Responder {
    run_js(JS_FILE)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(run)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}