use std::fmt::Display;

use axum::{
    Router,
    body::Body,
    http::Method,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use chrono::NaiveDate;
use form_fields::from_form::FromForm;
use form_fields_macro::{FromForm, Selectable};
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
            println!("{:?}", inner.text);
            println!("{:?}", inner.text_optional);
            println!("{:?}", inner.password);
            println!("{:?}", inner.password_optional);
            println!("{:?}", inner.number);
            println!("{:?}", inner.number_optional);
            println!("{:?}", inner.date);
            println!("{:?}", inner.date_optional);
            println!("{:?}", inner.r#enum);
            println!("{:?}", inner.radio);
            println!("{:?}", inner.select);
            println!("{:?}", inner.select_optional);
            println!("{:?}", inner.multiselect);
            println!("{:?}", inner.checkbox);
            println!("{:?}", inner.checkbox_optional);
            println!("{:?}", inner.passthrough);
            println!("{:?}", inner.passthrough_option);
            println!("{:?}", inner.passthrough_vec);
            println!("{:?}", inner.passthrough_vec_option);
            // Here you would typically save the data to a database or perform some action
            return Redirect::to("/").into_response();
        } else {
            println!("Form validation failed");
        }
    } else {
        form.passthrough.intermediate = Some("Some passthrough value".to_string());
        form.passthrough_option.intermediate = Some("Some optional passthrough value".to_string());
        form.passthrough_vec.intermediate = vec!["Value 1".to_string(), "Value 2".to_string()];
        form.passthrough_vec_option.intermediate = vec![
            "Optional Value 1".to_string(),
            "Optional Value 2".to_string(),
        ];
    }
    html! {
        h1 { "Simple Form Example" }
        form method="POST" {
            (form.text)
            (form.text_optional)
            (form.password)
            (form.password_optional)
            (form.number)
            (form.number_optional)
            (form.date)
            (form.date_optional)
            (form.r#enum)
            (form.radio)
            (form.select)
            (form.select_optional)
            (form.multiselect)
            (form.checkbox)
            (form.checkbox_optional)
            input type="text" name=(form.passthrough.field_name) value=[form.passthrough.intermediate];
            input type="text" name=(form.passthrough_option.field_name) value=[form.passthrough_option.intermediate];
            @for value in &form.passthrough_vec.intermediate {
                input type="hidden" name=(form.passthrough_vec.field_name) value=(value);
            }
            @for value in &form.passthrough_vec_option.intermediate {
                input type="hidden" name=(form.passthrough_vec_option.field_name) value=(value);
            }
            input type="submit";
        }
    }
    .into_response()
}

#[derive(Debug, FromForm)]
struct Test {
    #[text_field(display_name = "Required Text", max_length = 50)]
    pub text: String,

    #[text_field(display_name = "Optional Text", max_length = 50)]
    pub text_optional: Option<String>,

    #[password_field(display_name = "Required Password", max_length = 50)]
    pub password: String,

    #[password_field(display_name = "Optional Password", max_length = 50)]
    pub password_optional: Option<String>,

    #[number_field(display_name = "Required Number (0-120)", min = 0, max = 120)]
    pub number: u8,

    #[number_field(display_name = "Optional Number (0-120)", min = 0, max = 120)]
    pub number_optional: Option<u8>,

    #[date_select(display_name = "Required Date", min = "1900-01-01", max = "2023-12-31")]
    pub date: NaiveDate,

    #[date_select(display_name = "Optional Date", min = "1900-01-01", max = "2023-12-31")]
    pub date_optional: Option<NaiveDate>,

    #[radio_button(display_name = "Enum Radio", options = [Cars::Audi, Cars::BMW], default_value = Cars::Audi)]
    pub r#enum: Cars,

    #[radio_button(display_name = "Primitive Radio", options = [42, 69], default_value = 42)]
    pub radio: u8,

    #[select(display_name = "Required Select", options = [Cars::Audi, Cars::BMW], default_value = Cars::Audi, placeholder = "-- Please choose an option --")]
    pub select: Cars,

    #[select(display_name = "Optional Select", options = [Cars::Audi, Cars::BMW], placeholder = "-- Please choose an option --")]
    pub select_optional: Option<Cars>,

    #[multiselect(
        display_name = "Multiselect",
        options = [Cars::Audi, Cars::BMW, Cars::Mercedes],
    )]
    pub multiselect: Vec<Cars>,

    #[checkbox(
        display_name = "Required (true) Checkbox",
        help_text = "Helpful text",
        checked = false,
        required_true
    )]
    pub checkbox: bool,

    #[checkbox(
        display_name = "Optional Checkbox",
        help_text = "Helpful text",
        checked = false
    )]
    pub checkbox_optional: Option<bool>,

    #[passthrough]
    pub passthrough: String,

    #[passthrough]
    pub passthrough_option: Option<String>,

    #[passthrough]
    pub passthrough_vec: Vec<String>,

    #[passthrough]
    pub passthrough_vec_option: Option<Vec<String>>,
}

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug, Selectable)]
enum Cars {
    Audi,
    BMW,
    Mercedes,
}

impl Display for Cars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cars::Audi => write!(f, "audi"),
            Cars::BMW => write!(f, "bmw"),
            Cars::Mercedes => write!(f, "mercedes"),
        }
    }
}

impl std::str::FromStr for Cars {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "audi" => Ok(Cars::Audi),
            "bmw" => Ok(Cars::BMW),
            "mercedes" => Ok(Cars::Mercedes),
            _ => Err(()),
        }
    }
}

impl maud::Render for Cars {
    fn render(&self) -> maud::Markup {
        match self {
            Cars::Audi => html! { "Audi" },
            Cars::BMW => html! { "BMW" },
            Cars::Mercedes => html! { "Mercedes" },
        }
    }
}
