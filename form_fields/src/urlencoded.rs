use axum::body::{Body, to_bytes};

use crate::from_form::FormSpec;

pub(crate) async fn parse_form_urlencoded<Form: FormSpec>(
    form: &mut Form,
    req: Body,
) -> Option<()> {
    let Ok(bytes) = to_bytes(req, u16::MAX as _).await else {
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
        let body_string = "field1=value1&field2=value2";
        let body = Body::from(body_string);
        let mut form = MockFormSpec::generate_spec();
        
        assert!(parse_form_urlencoded(&mut form, body).await.is_some());
        assert_eq!(form.field1.intermediate, Some("value1".to_string()));
        assert_eq!(form.field2.intermediate, Some("value2".to_string()));
        
        let inner = form.inner().unwrap();
        assert_eq!(inner.field1, "value1");
        assert_eq!(inner.field2, "value2");
    }

    #[tokio::test]
    async fn parse_failure() {
        let body_string = "field1=value1&field3=value3";
        let body = Body::from(body_string);
        let mut form = MockFormSpec::generate_spec();
        
        assert!(parse_form_urlencoded(&mut form, body).await.is_none());
        assert_eq!(form.field1.intermediate, Some("value1".to_string()));
        assert_eq!(form.field2.intermediate, None);
    
        assert!(form.inner().is_none());
    }

    #[tokio::test]
    async fn parse_partial_failure() {
        let body_string = "field1=value1";
        let body = Body::from(body_string);
        let mut form = MockFormSpec::generate_spec();
        
        assert!(parse_form_urlencoded(&mut form, body).await.is_some());
        assert_eq!(form.field1.intermediate, Some("value1".to_string()));
        assert_eq!(form.field2.intermediate, None);

        assert!(form.inner().is_none());
    }

    #[tokio::test]
    async fn parse_empty() {
        let body_string = "";
        let body = Body::from(body_string);
        let mut form = MockFormSpec::generate_spec();
        
        assert!(parse_form_urlencoded(&mut form, body).await.is_some());
        assert_eq!(form.field1.intermediate, None);
        assert_eq!(form.field2.intermediate, None);

        assert!(form.inner().is_none());
    }
}
