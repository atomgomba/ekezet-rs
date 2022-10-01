use actix_web::{get, http, middleware, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sycamore::{render_to_string, view};

use crate::cfg;
use crate::cfg::Link;
use crate::html::IndexPage;

struct State {
    links: Vec<Link>,
}

pub fn run_server(host: &str, port: u16, use_ssl: bool) -> std::io::Result<()> {
    actix_web::rt::System::new().block_on(async move {
        let server = HttpServer::new(|| {
            App::new()
                .wrap(middleware::Compress::default())
                .app_data(web::Data::new(State {
                    links: cfg::load_links(),
                }))
                .service(root)
                .service(favicon)
                .service(js)
                .service(wasm)
        });
        if use_ssl {
            let mut builder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls())?;
            builder.set_private_key_file("app.key", SslFiletype::PEM)?;
            builder.set_certificate_chain_file("app.crt")?;
            server.bind_openssl(format!("{host}:{port}"), builder)?
        } else {
            server.bind((host, port))?
        }
        .run()
        .await
    })
}

#[get("/")]
async fn root(data: web::Data<State>) -> impl Responder {
    let preamble = include_str!("preamble.in");
    let body = render_to_string(|cx| {
        view! { cx,
            IndexPage(links=data.links.clone())
        }
    });

    HttpResponse::Ok()
        .content_type(http::header::ContentType(mime::TEXT_HTML_UTF_8))
        .body(format!("{preamble}{body}"))
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "image/x-icon"))
        .body(include_bytes!("../res/favicon.ico").as_slice())
}

#[get("/lib.js")]
async fn js() -> impl Responder {
    HttpResponse::Ok()
        .content_type(http::header::ContentType(mime::TEXT_JAVASCRIPT))
        .body(include_bytes!("../target/lib.min.js").as_slice())
}

#[get("/lib_bg.wasm")]
async fn wasm() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/wasm"))
        .body(include_bytes!("../lib/pkg/lib_bg.wasm").as_slice())
}
