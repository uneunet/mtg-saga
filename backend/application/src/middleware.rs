use anyhow::Result;
use axum::extract::State;
use axum::response::Response;
use axum::{extract::Request, http::StatusCode, middleware::Next};
use axum_extra::extract::CookieJar;
use usecase::UseCase;

pub async fn auth(
    State(use_case): State<UseCase>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(token) = jar.get("token") {
        let session = use_case
            .session
            .find_by_token(token.value().to_string())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;
        request.extensions_mut().insert(session.user_id);
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
