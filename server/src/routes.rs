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
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use gh_pilot::ghp_api::webhooks::GithubEvent;
use log::*;

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("👍")
}

#[post("/webhook")]
pub async fn github_webhook(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    debug!("Headers: {:?}", req.headers());
    let payload = match std::str::from_utf8(body.as_ref()) {
        Ok(text) => text,
        Err(_) => return HttpResponse::BadRequest().into(),
    };
    debug!("Event: {:?}", payload);
    let event_name = match req
        .headers()
        .get("x-github-event")
        .map(|v| v.to_str())
        .and_then(Result::ok)
    {
        Some(v) => v,
        None => return HttpResponse::BadRequest().into(),
    };

    let event = GithubEvent::from_webhook_info(event_name, payload);
    // TODO - set secret on webhook and validate signature
    HttpResponse::Ok().json(event)
}
