use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use crate::web::{Error, Result, AUTH_TOKEN};
use crate::ctx::Ctx;
use crate::model::ModelManager;

pub async fn mw_ctx_require<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_require", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve<B>(
    _mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let _auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = Ctx::new(100).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()));

    if result_ctx.is_err()
        && !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie))
    {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        match parts
            .extensions
            .get::<CtxExtResult>()
        {
            Some(Ok(ctx)) => Ok(ctx.clone()),
            None => Err(Error::CtxExt(CtxExtError::CtxNotInRequestExt)),
            Some(Err(e)) => Err(Error::CtxExt(e.clone())),
        }
    }
}

type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    CtxNotInRequestExt,
    CtxCreateFail(String),
}