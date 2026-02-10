use tower_cookies::cookie::{Cookie, SameSite, time::Duration};

pub fn set_refresh_cookie(token: String) -> Cookie<'static> {
    Cookie::build(("refresh_token", token))
        .secure(true) // true in production (HTTPS)
        .http_only(true)
        .same_site(SameSite::None)
        .domain("actor-portfolio.onrender.com")
        .path("/")
        .max_age(Duration::days(1))
        .build()
}

pub fn clear_refresh_cookies() -> Cookie<'static> {
    Cookie::build(("refresh_token", ""))
        .secure(true) // true in production (HTTPS)
        .http_only(true)
        .same_site(SameSite::None)
        .domain("actor-portfolio.onrender.com")
        .path("/")
        .max_age(Duration::days(30))
        .build()
}
// .domain("www.rust-lang.org")
