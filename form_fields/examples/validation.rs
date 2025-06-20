use axum::{
    Router,
    body::Body,
    http::Method,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use form_fields::from_form::FromForm;
use form_fields_macro::FromForm;
use maud::html;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(validation))
        .route("/", post(validation));

    let listen_addr = "localhost:8080";
    println!("Listening on http://{}", listen_addr);

    let listener = TcpListener::bind(listen_addr).await?;
    axum::serve(listener, app.into_make_service()).await
}

// To add custom validation, we have to add error messages to our form struct,
// therefore it's mutable here.
async fn validation(method: Method, FromForm(mut form): FromForm<Test>) -> Response<Body> {
    if method == Method::POST {
        println!("Form submitted");
        if let Some(inner) = form.inner() {
            println!("Form data: {:?}", inner);
            if inner.text != "valid" {
                form.text.set_error("Text must be 'valid'".to_string());
            } else {
                // Here you would typically save the data to a database or perform some action
                return Redirect::to("/").into_response();
            }
        } else {
            println!("Form validation failed");
        }
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
    pub text: String,
}
