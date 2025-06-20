use axum::body::Body;
use multer::Multipart;

use crate::from_form::FormSpec;

pub(crate) async fn parse_multipart<Form: FormSpec>(
    form: &mut Form,
    body: Body,
    content_type: &str,
) -> Option<()> {
    let boundary = multer::parse_boundary(content_type).ok()?;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate as form_fields;
    use axum::body::Body;
    use form_fields_macro::FromForm;

    #[derive(FromForm)]
    struct Mock {
        #[text_field(display_name = "Field 1")]
        field1: String,
        #[text_field(display_name = "Field 2")]
        field2: String,
    }

    #[tokio::test]
    async fn parse_success() {
        let body_string = "--boundary\r\n\
Content-Disposition: form-data; name=\"field1\"\r\n\r\nvalue1\r\n--boundary\r\n\
Content-Disposition: form-data; name=\"field2\"\r\n\r\nvalue2\r\n--boundary--\r\n";
        let body = Body::from(body_string);

        let mut form = MockFormSpec::new();
        let content_type = "multipart/form-data; boundary=boundary";

        let result = parse_multipart(&mut form, body, content_type).await;
        assert!(result.is_some());
        assert_eq!(form.field1.intermediate, Some("value1".to_string()));
        assert_eq!(form.field2.intermediate, Some("value2".to_string()));

        let inner = form.inner().unwrap();
        assert_eq!(inner.field1, "value1");
        assert_eq!(inner.field2, "value2");
    }

    #[tokio::test]
    async fn parse_failure() {
        let body_string = "--boundary\r\n\
Content-Disposition: form-data; name=\"field1\"\r\n\r\nvalue1\r\n--boundary\r\n\
Content-Disposition: form-data; name=\"field3\"\r\n\r\nvalue3\r\n--boundary--\r\n";
        let body = Body::from(body_string);

        let mut form = MockFormSpec::new();
        let content_type = "multipart/form-data; boundary=boundary";

        assert!(parse_multipart(&mut form, body, content_type).await.is_none());
        assert_eq!(form.field1.intermediate, Some("value1".to_string()));
        assert_eq!(form.field2.intermediate, None);

        assert!(form.inner().is_none());
    }

    #[tokio::test]
    async fn parse_partial_failure() {
        let body_string = "--boundary\r\n\
Content-Disposition: form-data; name=\"field1\"\r\n\r\nvalue1\r\n--boundary--\r\n";
        let body = Body::from(body_string);

        let mut form = MockFormSpec::new();
        let content_type = "multipart/form-data; boundary=boundary";

        assert!(parse_multipart(&mut form, body, content_type).await.is_some());
        assert_eq!(form.field1.intermediate, Some("value1".to_string()));
        assert_eq!(form.field2.intermediate, None);

        assert!(form.inner().is_none());
    }
    
    #[tokio::test]
    async fn parse_empty() {
        let body_string = "--boundary--\r\n";
        let body = Body::from(body_string);

        let mut form = MockFormSpec::new();
        let content_type = "multipart/form-data; boundary=boundary";
        
        assert!(parse_multipart(&mut form, body, content_type).await.is_some());
        assert_eq!(form.field1.intermediate, None);
        assert_eq!(form.field2.intermediate, None);

        assert!(form.inner().is_none());
    }
}