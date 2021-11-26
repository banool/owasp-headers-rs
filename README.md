# owasp-headers-rs [![Status](https://img.shields.io/badge/status-actively--developed-brightgreen)](https://gitlab.com/jokeyrhyme/owasp-headers-rs) [![Gitlab pipeline status](https://img.shields.io/gitlab/pipeline-status/jokeyrhyme/owasp-headers-rs?branch=main)](https://gitlab.com/jokeyrhyme/owasp-headers-rs/-/pipelines?ref=main) [![Crates.io](https://img.shields.io/crates/v/owasp-headers)](https://crates.io/crates/owasp-headers) [![docs.rs](https://img.shields.io/docsrs/owasp-headers)](https://docs.rs/owasp-headers)

best-practice OWASP HTTP response headers ( https://owasp.org/www-project-secure-headers/ ) for Rust

## [headers](https://gitlab.com/jokeyrhyme/owasp-headers-rs/-/blob/main/fixtures/headers.toml)

```
HTTP-Strict-Transport-Security = "max-age=31536000 ; includeSubDomains"
X-Frame-Options = "deny"
X-Content-Type-Options = "nosniff"
Content-Security-Policy = "default-src 'self'; object-src 'none'; child-src 'self'; frame-ancestors 'none'; upgrade-insecure-requests; block-all-mixed-content"
X-Permitted-Cross-Domain-Policies = "none"
Referrer-Policy = "no-referrer"
Clear-Site-Data = "\"cache\",\"cookies\",\"storage\""
Cross-Origin-Embedder-Policy = "require-corp"
Cross-Origin-Opener-Policy = "same-origin"
Cross-Origin-Resource-Policy = "same-origin"
Permissions-Policy = "accelerometer=(),autoplay=(),camera=(),display-capture=(),document-domain=(),encrypted-media=(),fullscreen=(),geolocation=(),gyroscope=(),magnetometer=(),microphone=(),midi=(),payment=(),picture-in-picture=(),publickey-credentials-get=(),screen-wake-lock=(),sync-xhr=(self),usb=(),web-share=(),xr-spatial-tracking=()"
Cache-Control = "no-store, max-age=0"
Pragma = "no-cache"
```

## see also

- https://owasp.org/www-project-secure-headers/

- tower-default-headers: [source code](https://gitlab.com/jokeyrhyme/tower-default-headers-rs), [crate](https://crates.io/crates/tower-default-headers)

