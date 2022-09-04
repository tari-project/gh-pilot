//! Request handler definitions
//!
//! Define each route and it handler here.
//! Handlers that are more than a line or two MUST go into a separate module. Keep this module neat and tidy 🙏
//!
//! A note about performance:
//! Since each worker thread processes its requests sequentially, handlers which block the current thread will cause the
//! current worker to stop processing new requests:
//!
//!     fn my_handler() -> impl Responder {
//!         std::thread::sleep(Duration::from_secs(5)); // <-- Bad practice! Will cause the current worker thread to
//! hang!         "response"
//!     }
//! For this reason, any long, non-cpu-bound operation (e.g. I/O, database operations, etc.) should be expressed as
//! futures or asynchronous functions. Async handlers get executed concurrently by worker threads and thus don’t block
//! execution:
//!
//!     async fn my_handler() -> impl Responder {
//!         tokio::time::sleep(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
//!         "response"
//!     }
use actix_web::{get, http::header::HeaderMap, post, web, HttpRequest, HttpResponse, Responder};
use gh_pilot::ghp_api::webhooks::GithubEvent;
use log::*;

use crate::error::ServerError;

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("👍")
}

#[post("/webhook")]
pub async fn github_webhook(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, ServerError> {
    let headers = req.headers();
    debug!("Received webhook request");
    check_valid_signature(headers)?;
    debug!("Received webhook signature check passed");
    let payload = std::str::from_utf8(body.as_ref()).map_err(|e| ServerError::InvalidRequestBody(e.to_string()))?;
    debug!("Decoded payload body");
    let event_name = headers
        .get("x-github-event")
        .ok_or(ServerError::InvalidEventHeader("x-github-event is missing".into()))?
        .to_str()
        .map_err(|_| ServerError::InvalidEventHeader("x-github-event is not a valid string".into()))?;
    debug!("Extracted event name");
    let event = GithubEvent::from_webhook_info(event_name, payload);
    info!("Github event received: {}, {}", event_name, event.summary());
    Ok(HttpResponse::Ok().finish())
}

fn check_valid_signature(_headers: &HeaderMap) -> Result<(), ServerError> {
    // TODO
    Ok(())
}
