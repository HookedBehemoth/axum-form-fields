## Form Fields
[![crates.io](https://img.shields.io/crates/v/form_fields.svg)](https://crates.io/crates/form_fields)
![Crates.io Size](https://img.shields.io/crates/size/form_fields)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/form_fields)
[![docs.rs](https://docs.rs/form_fields/badge.svg)](https://docs.rs/form_fields)
[![License: GNU JPL](https://img.shields.io/badge/License-GNU%20JPL-blue.svg)](http://tom7.org/bovex/JCOPYING)

Helper crate for working with HTML forms in [axum](https://github.com/tokio-rs/axum/).

Simplifies the parsing and validating user input for creating new and editing existing data in your application.

## Documentation
Learn about all the possible macro attributes [here](https://docs.rs/form_fields_macro/latest/form_fields_macro/derive.FromForm.html).

## Examples
In [The Examples folder](form_fields/examples) folder, you can find a few basic examples.

## Basic usage
Derive from "FromForm" on a struct with the data you allow the user to submit.
```rs
#[derive(Debug, FromForm)]
struct Test {
    #[text_field(display_name = "Required Text", max_length = 50)]
    text: String,
}
```
This will generate a Form-Spec, which can be retrieved in your axum-handlers.
```rs
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
```
Currently, only [maud](https://maud.lambda.xyz/) is supported, but all data is exposed so rendering the inputs in any other markup generator or even altering the format is possible.

## Goals for stable release

- [ ] documentation
- [ ] better naming (help needed)
    - [ ] descriptor
- [ ] feature segregation
    - [ ] multer & form_urlencoded
    - [ ] axum
    - [ ] renderers
    - [ ] chrono
- [ ] file handling
    - [ ] loaded fully
    - [ ] async user handled
- [ ] HTML renderers
    - [x] maud
- [ ] clean up macro code
    - [ ] clearer error handling
