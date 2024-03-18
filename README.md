# owasp-headers-rs

Best-practice OWASP HTTP response headers (https://owasp.org/www-project-secure-headers/) for Rust

Forked from: https://gitlab.com/jokeyrhyme/owasp-headers-rs. Updated to be compatible with Axum 0.7 and friends. If you're reading this in the future, see [this issue](https://gitlab.com/jokeyrhyme/owasp-headers-rs/-/issues/1) to see if the original repo has been updated.

## [Headers](https://gitlab.com/jokeyrhyme/owasp-headers-rs/-/blob/main/fixtures/headers.toml)

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

## See also

- https://owasp.org/www-project-secure-headers/

- tower-default-headers: https://github.com/banool/tower-default-headers-rs
