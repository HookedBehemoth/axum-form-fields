use axum::{
    Router,
    body::Body,
    http::Method,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use form_fields::from_form::FromForm;
use form_fields_macro::FromForm;
use maud::html;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(simple))
        .route("/", post(simple));

    let listen_addr = "localhost:8080";
    println!("Listening on http://{}", listen_addr);

    let listener = TcpListener::bind(listen_addr).await?;
    axum::serve(listener, app.into_make_service()).await
}

async fn simple(method: Method, FromForm(mut form): FromForm<Test>) -> Response<Body> {
    if method == Method::POST {
        let inner = form.inner().unwrap();
        println!("Form submitted: text: {}", inner.text);
    }
    html! {
        h1 { "Simple Form Example" }
        form method="POST" {
            (form.text)
            input type="submit";
        }
    }
    .into_response()
}

#[derive(Debug, FromForm)]
struct Test {
    #[text_field(display_name = "Required Text", max_length = 50)]
    text: String,
}
