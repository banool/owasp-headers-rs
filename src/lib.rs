#![deny(clippy::all, unsafe_code)]
#![warn(missing_docs)]

//! Modern web browsers may prevent or mitigate security vulnerabilities when they encounter the
//! [HTTP response headers recommended by OWASP](https://owasp.org/www-project-secure-headers/).
//!
//! This crate offers these HTTP headers and their values,
//! so that they may be more-conveniently used when developing web services in Rust.
//!
//! Example:
//! ```
//! use axum::body::Body;
//! use http::response::Parts;
//! use hyper::{Request, Response};
//! use std::convert::Infallible;
//!
//! async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//!     let mut response = Response::new(Body::from("hello, world!"));
//!     let mut headers = response.headers_mut();
//!     headers.extend(owasp_headers::headers());
//!     Ok(response)
//! }
//! ```
//!
//! If you are developing a web service using
//! [`tower::Service`](https://docs.rs/tower/0.4.11/tower/trait.Service.html),
//! you may find it even more convenient to apply these headers using
//! [tower-default-headers](https://crates.io/crates/tower-default-headers).

use http::header::{
    HeaderMap, HeaderValue, CACHE_CONTROL, CONTENT_SECURITY_POLICY, PRAGMA, REFERRER_POLICY,
    X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS,
};

/// produces an owned-collection of headers and their values
pub fn headers() -> HeaderMap {
    let mut h = HeaderMap::new();
    h.reserve(13);
    h.insert(
        HTTP_STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static(HTTP_STRICT_TRANSPORT_SECURITY_DEFAULT),
    );
    h.insert(
        X_FRAME_OPTIONS,
        HeaderValue::from_static(X_FRAME_OPTIONS_DEFAULT),
    );
    h.insert(
        X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static(X_CONTENT_TYPE_OPTIONS_DEFAULT),
    );
    h.insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(CONTENT_SECURITY_POLICY_DEFAULT),
    );
    h.insert(
        X_PERMITTED_CROSS_DOMAIN_POLICIES,
        HeaderValue::from_static(X_PERMITTED_CROSS_DOMAIN_POLICIES_DEFAULT),
    );
    h.insert(
        REFERRER_POLICY,
        HeaderValue::from_static(REFERRER_POLICY_DEFAULT),
    );
    h.insert(
        CLEAR_SITE_DATA,
        HeaderValue::from_static(CLEAR_SITE_DATA_DEFAULT),
    );
    h.insert(
        CROSS_ORIGIN_EMBEDDER_POLICY,
        HeaderValue::from_static(CROSS_ORIGIN_EMBEDDER_POLICY_DEFAULT),
    );
    h.insert(
        CROSS_ORIGIN_OPENER_POLICY,
        HeaderValue::from_static(SAME_ORIGIN),
    );
    h.insert(
        CROSS_ORIGIN_RESOURCE_POLICY,
        HeaderValue::from_static(SAME_ORIGIN),
    );
    h.insert(
        PERMISSIONS_POLICY,
        HeaderValue::from_static(PERMISSIONS_POLICY_DEFAULT),
    );
    h.insert(
        CACHE_CONTROL,
        HeaderValue::from_static(CACHE_CONTROL_DEFAULT),
    );
    h.insert(PRAGMA, HeaderValue::from_static(PRAGMA_DEFAULT));
    h
}

const CACHE_CONTROL_DEFAULT: &str = "no-store, max-age=0";
const CLEAR_SITE_DATA: &str = "clear-site-data";
const CLEAR_SITE_DATA_DEFAULT: &str = r#""cache","cookies","storage""#;
const CONTENT_SECURITY_POLICY_DEFAULT: &str = "default-src 'self'; object-src 'none'; child-src 'self'; frame-ancestors 'none'; upgrade-insecure-requests; block-all-mixed-content";
const CROSS_ORIGIN_EMBEDDER_POLICY: &str = "cross-origin-embedder-policy";
const CROSS_ORIGIN_EMBEDDER_POLICY_DEFAULT: &str = "require-corp";
const CROSS_ORIGIN_OPENER_POLICY: &str = "cross-origin-opener-policy";
const CROSS_ORIGIN_RESOURCE_POLICY: &str = "cross-origin-resource-policy";
const HTTP_STRICT_TRANSPORT_SECURITY: &str = "http-strict-transport-security";
const HTTP_STRICT_TRANSPORT_SECURITY_DEFAULT: &str = "max-age=31536000 ; includeSubDomains";
const PERMISSIONS_POLICY: &str = "permissions-policy";
const PERMISSIONS_POLICY_DEFAULT: &str = "accelerometer=(),autoplay=(),camera=(),display-capture=(),document-domain=(),encrypted-media=(),fullscreen=(),geolocation=(),gyroscope=(),magnetometer=(),microphone=(),midi=(),payment=(),picture-in-picture=(),publickey-credentials-get=(),screen-wake-lock=(),sync-xhr=(self),usb=(),web-share=(),xr-spatial-tracking=()";
const PRAGMA_DEFAULT: &str = "no-cache";
const REFERRER_POLICY_DEFAULT: &str = "no-referrer";
const SAME_ORIGIN: &str = "same-origin";
const X_CONTENT_TYPE_OPTIONS_DEFAULT: &str = "nosniff";
const X_FRAME_OPTIONS_DEFAULT: &str = "deny";
const X_PERMITTED_CROSS_DOMAIN_POLICIES: &str = "x-permitted-cross-domain-policies";
const X_PERMITTED_CROSS_DOMAIN_POLICIES_DEFAULT: &str = "none";

#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::Path};

    use toml::value::Value;

    use super::*;

    #[test]
    fn headers_returns_headermap_with_expected_len() {
        let got = headers();
        assert_eq!(got.len(), 13);
    }

    #[test]
    fn headers_returns_headermap_with_expected_contents() {
        let got = headers();

        let fixture_path = Path::new("./fixtures/headers.toml");
        let fixture_data =
            read_to_string(fixture_path).expect("could not read fixtures/headers.toml");
        let fixture = fixture_data.parse::<Value>().unwrap();

        if let Value::Table(table) = fixture {
            assert_eq!(table.len(), 13);
            for (name, value) in table.iter() {
                assert_eq!(got[name], value.as_str().unwrap());
            }
        } else {
            panic!("unexpected TOML structure");
        }
    }
}
