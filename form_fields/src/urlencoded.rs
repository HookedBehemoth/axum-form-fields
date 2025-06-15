use axum::{
    body::{Body, to_bytes},
    extract::Request,
};

use crate::from_form::FormSpec;

pub(crate) async fn parse_form_urlencoded<Form: FormSpec>(
    form: &mut Form,
    req: Request<Body>,
) -> Option<()> {
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
