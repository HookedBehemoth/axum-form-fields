use axum::{
    body::{Body, to_bytes},
    extract::Request,
};
use multer::Multipart;

pub trait FormSpecable {
    type Spec: FormSpec;
}

pub trait FormSpec: Send {
    fn generate_spec() -> Self;
    fn parse_field(&mut self, name: &str, value: &str) -> bool;
}

pub struct FromForm<T>(pub T::Spec)
where
    T: FormSpecable;

impl<Specable, State> axum::extract::FromRequest<State> for FromForm<Specable>
where
    Specable: FormSpecable,
    State: Send + Sync,
{
    type Rejection = maud::Markup;

    async fn from_request(
        req: axum::extract::Request<axum::body::Body>,
        _state: &State,
    ) -> Result<Self, Self::Rejection> {
        let method = req.method().clone();
        let mut generated = Specable::Spec::generate_spec();

        if method == axum::http::Method::POST {
            if parse_request_body(&mut generated, req).await.is_none() {
                return Err(maud::html! { ("Failed to load Form contents") });
            }

            Ok(Self(generated))
        } else {
            Ok(Self(generated))
        }
    }
}

async fn parse_request_body<Form: FormSpec>(
    form: &mut Form,
    req: axum::extract::Request<axum::body::Body>,
) -> Option<()> {
    let content_type = req
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .cloned()?
        .to_str()
        .ok()?
        .to_string();

    log::debug!("content_type: {}", content_type);
    match &content_type {
        content_type if content_type == "application/x-www-form-urlencoded" => {
            parse_form_urlencoded(form, req).await
        }
        content_type if content_type.starts_with("multipart/form-data") => {
            // Handle multipart form data if necessary
            parse_multipart(form, req, content_type).await
        }
        _ => {
            // Handle other content types if necessary
            None
        }
    }
}

async fn parse_form_urlencoded<Form: FormSpec>(form: &mut Form, req: Request<Body>) -> Option<()> {
    let Ok(bytes) = to_bytes(req.into_body(), u16::MAX as _).await else {
        return None;
    };

    let text = String::from_utf8(bytes.to_vec()).ok()?;
    log::debug!("Parsing form-urlencoded data: {}", text);

    let parsed = form_urlencoded::parse(&bytes);

    for (key, value) in parsed {
        log::debug!("Parsing field: {} = {}", key, value);
        if !form.parse_field(&key, &value) {
            return None;
        }
    }

    Some(())
}

async fn parse_multipart<Form: FormSpec>(
    form: &mut Form,
    req: Request<Body>,
    content_type: &str,
) -> Option<()> {
    let boundary = multer::parse_boundary(content_type).ok()?;
    let body = req.into_body();

    // Create a Multipart parser
    let mut multipart = Multipart::new(body.into_data_stream(), boundary);

    while let Some(field) = multipart.next_field().await.ok()? {
        let Some(name) = field.name() else {
            continue;
        };
        let name = name.to_string();
        log::debug!("Parsing field: {}", name);

        if let Some(file_name) = field.file_name() {
            log::warn!("File not supported: {}", file_name);
            continue;
        }

        if let Ok(text) = field.text().await {
            log::debug!("Field text: {}", text);
            if !form.parse_field(&name, &text) {
                log::error!("Failed to parse: {}", name);
                return None;
            }
        }
    }

    Some(())
}
