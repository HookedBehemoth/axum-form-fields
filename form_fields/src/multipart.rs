use axum::{body::Body, extract::Request};
use multer::Multipart;

use crate::from_form::FormSpec;

pub(crate) async fn parse_multipart<Form: FormSpec>(
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
