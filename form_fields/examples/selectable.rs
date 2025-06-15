use axum::{
    Router,
    body::Body,
    http::Method,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use form_fields::{from_form::FromForm, selectable::Selectable};
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
    // Load all options from the database or any other source
    let options = vec![
        MyEnum("Option 1".to_string()),
        MyEnum("Option 2".to_string()),
        MyEnum("Option 3".to_string()),
    ];
    form.radio.descriptor.options = options.clone();
    form.select.descriptor.options = options.clone();

    if method == Method::POST {
        if let Some(inner) = form.inner() {
            println!("{:?}", inner.radio);
            println!("{:?}", inner.select);
            // Here you would typically save the data to a database or perform some action
            return Redirect::to("/").into_response();
        } else {
            println!("Form validation failed");
        }
    }
    html! {
        h1 { "Simple Form Example" }
        form method="POST" {
            (form.radio)
            input type="submit";
        }
    }
    .into_response()
}

#[derive(Debug, FromForm)]
struct Test {
    #[radio_button(display_name = "Radio", options = [], default_value = MyEnum(String::new()))]
    pub radio: MyEnum,

    #[select(display_name = "Select", options = [], default_value = MyEnum(String::new()), placeholder = "-- Please choose an option --")]
    pub select: Option<MyEnum>,
}

#[derive(Debug, Clone)]
struct MyEnum(String);

impl Selectable for MyEnum {
    type Key = String;
    type DisplayValue = String;

    fn key(&self) -> Self::Key {
        self.0.clone()
    }
    fn display_value(&self) -> Self::DisplayValue {
        self.0.clone()
    }
}
