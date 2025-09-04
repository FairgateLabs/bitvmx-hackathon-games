use axum::{
    body::{to_bytes, Body},
    http::{Request, Response},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::info;

/// Middleware que loguea bodies (dentro del span actual creado por TraceLayer).
#[derive(Clone)]
pub struct LoggingLayer {
    max_log_bytes: usize,
}

impl LoggingLayer {
    pub fn new(max_log_bytes: usize) -> Self {
        Self { max_log_bytes }
    }
}

#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    inner: S,
    max_log_bytes: usize,
}

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingMiddleware {
            inner,
            max_log_bytes: self.max_log_bytes,
        }
    }
}

impl<S> Service<Request<Body>> for LoggingMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    S::Future: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        let max = self.max_log_bytes;

        Box::pin(async move {
            // ---- Request ----
            let (parts, body_in) = req.into_parts();
            let bytes = to_bytes(body_in, usize::MAX).await.unwrap_or_default();
            let body_str = String::from_utf8_lossy(&bytes);
            let truncated = &body_str[..body_str.len().min(max)];
            info!("👉 Request body (truncated): {}", truncated);

            let req2 = Request::from_parts(parts, Body::from(bytes));
            let res = inner.call(req2).await?;

            // ---- Response ----
            let (parts, body_out) = res.into_parts();
            let resp_bytes = to_bytes(body_out, usize::MAX).await.unwrap_or_default();
            let resp_str = String::from_utf8_lossy(&resp_bytes);
            let resp_trunc = &resp_str[..resp_str.len().min(max)];
            info!("👈 Response body (truncated): {}", resp_trunc);

            let res2 = Response::from_parts(parts, Body::from(resp_bytes));
            Ok(res2)
        })
    }
}
