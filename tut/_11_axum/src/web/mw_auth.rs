use axum::body::Body;
use axum::{extract::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;
use crate::{Error,Result};
use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
        cookies: Cookies,
        req: Request<Body>, 
        next: Next
    ) -> Result<Response> {
    
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoTokenCookie)?;
    Ok(next.run(req).await)
}