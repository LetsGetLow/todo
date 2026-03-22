pub struct HeaderToken {
    pub message: String,
}

#[cfg(feature = "actix")]
mod actix_impl {
    use super::HeaderToken;
    use crate::errors::{NanoServiceError, NanoServiceErrorStatus};
    pub use actix_web::{FromRequest as ActixFromRequest, HttpRequest, dev::Payload};
    use futures::future::{Ready, err, ok};

    impl ActixFromRequest for HeaderToken {
        type Error = NanoServiceError;
        type Future = Ready<Result<HeaderToken, NanoServiceError>>;
        fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
            let raw_data = match req.headers().get("Token") {
                Some(data) => data,
                None => {
                    return err(NanoServiceError::new(
                        "Token not in header under the key 'Token'".to_string(),
                        NanoServiceErrorStatus::Unauthorized,
                    ));
                }
            };

            let message = match raw_data.to_str() {
                Ok(token) => token.to_string(),
                Err(_) => {
                    return err(NanoServiceError::new(
                        "Token not a valid string".to_string(),
                        NanoServiceErrorStatus::Unauthorized,
                    ));
                }
            };

            ok(HeaderToken { message })
        }
    }
}

// Re-exporting the ActixFromRequest trait for use in actix web servers layer whithout clash with other web frameworks layers.
#[cfg(feature = "actix")]
pub use actix_impl::ActixFromRequest;
