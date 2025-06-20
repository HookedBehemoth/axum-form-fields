/// Either urlencoded or multipart has to be enabled
#[cfg(not(any(feature = "urlencoded", feature = "multipart")))]
compile_error!("Either the 'urlencoded' or 'multipart' feature must be enabled.");

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
    let body = req.into_body();
    match &content_type {
        #[cfg(feature = "urlencoded")]
        content_type if content_type == "application/x-www-form-urlencoded" => {
            crate::urlencoded::parse_form_urlencoded(form, body).await
        }
        #[cfg(feature = "multipart")]
        content_type if content_type.starts_with("multipart/form-data") => {
            crate::multipart::parse_multipart(form, body, content_type).await
        }
        _ => {
            // Handle other content types if necessary
            None
        }
    }
}
