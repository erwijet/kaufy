use poem::http::header::AUTHORIZATION;
use poem::{Endpoint, Middleware, Request, Result};

use crate::jwt;

/// A middleware that extract token from HTTP headers. See https://docs.rs/poem/latest/poem/middleware/trait.Middleware.html
pub struct JwtMiddleware;

impl<E: Endpoint> Middleware<E> for JwtMiddleware {
    type Output = JwtMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        JwtMiddlewareImpl { ep }
    }
}

pub struct JwtMiddlewareImpl<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for JwtMiddlewareImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        // Just example. You have to make sure your middleware is correct
        if let Some(value) = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .filter(|value| value.starts_with("Bearer "))
            .map(|value| &value[7..])
        {
            // Decode JWT token
            let claims = jwt::decode_jwt(value)?;
            req.extensions_mut().insert(claims);
        }

        // call the next endpoint.
        self.ep.call(req).await
    }
}