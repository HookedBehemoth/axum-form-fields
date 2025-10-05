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
        .route("/", get(simple))
        .route("/", post(simple));

    let listen_addr = "localhost:8080";
    println!("Listening on http://{}", listen_addr);

    let listener = TcpListener::bind(listen_addr).await?;
    axum::serve(listener, app.into_make_service()).await
}

async fn simple(method: Method, FromForm(mut form): FromForm<Test>) -> Response<Body> {
    if method == Method::POST {
        println!("Form submitted");
        if let Some(inner) = form.inner() {
            println!("{:?}", inner.passthrough);
            println!("{:?}", inner.passthrough_option);
            println!("{:?}", inner.passthrough_vec);
            println!("{:?}", inner.passthrough_vec_option);
            // Here you would typically save the data to a database or perform some action
            if !inner.roundtrip {
                return Redirect::to("/").into_response();
            }
        } else {
            println!("Form validation failed");
        }
    } else {
        let test = Test {
            roundtrip: true,
            // Has to be filled manually here, rendered manually or added through javascript
            passthrough: "Some passthrough value".to_string(),
            passthrough_option: Some("Some optional passthrough value".to_string()),
            passthrough_vec: vec!["Value 1".to_string(), "Value 2".to_string()],
            passthrough_vec_option: Some(vec![
                "Optional Value 1".to_string(),
                "Optional Value 2".to_string(),
            ]),
        };
        form.load(test);
        // form.passthrough.intermediate = Some("Some passthrough value".to_string());
        // form.passthrough_option.intermediate = Some("Some optional passthrough value".to_string());
        // form.passthrough_vec.intermediate = vec!["Value 1".to_string(), "Value 2".to_string()];
        // form.passthrough_vec_option.intermediate = vec![
        //     "Optional Value 1".to_string(),
        //     "Optional Value 2".to_string(),
        // ];
    }
    html! {
        h1 { "Simple Form Example" }
        form method="POST" {
            (form.roundtrip)
            figure {
                input type="text" name=(form.passthrough.field_name) value=[form.passthrough.intermediate];
                figcaption { (form.passthrough.display_name) }
            }
            figure {
                input type="text" name=(form.passthrough_option.field_name) value=[form.passthrough_option.intermediate];
                figcaption { (form.passthrough_option.display_name) }
            }
            figure {
                @for value in &form.passthrough_vec.intermediate {
                    input type="text" name=(form.passthrough_vec.field_name) value=(value);
                }
                figcaption { (form.passthrough_vec.display_name) }
            }
            figure {
                @for value in &form.passthrough_vec_option.intermediate {
                    input type="text" name=(form.passthrough_vec_option.field_name) value=(value);
                }
                figcaption { (form.passthrough_vec_option.display_name) }
            }
            input type="submit";
        }
    }
    .into_response()
}

#[derive(Debug, FromForm)]
struct Test {
    #[checkbox]
    pub roundtrip: bool,

    #[passthrough]
    pub passthrough: String,

    #[passthrough]
    pub passthrough_option: Option<String>,

    #[passthrough]
    pub passthrough_vec: Vec<String>,

    #[passthrough]
    pub passthrough_vec_option: Option<Vec<String>>,
}
