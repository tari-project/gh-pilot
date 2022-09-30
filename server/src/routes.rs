//! Request handler definitions
//!
//! Define each route and it handler here.
//! Handlers that are more than a line or two MUST go into a separate module. Keep this module neat and tidy 🙏
//!
//! A note about performance:
//! Since each worker thread processes its requests sequentially, handlers which block the current thread will cause the
//! current worker to stop processing new requests:
//! ```nocompile
//!     fn my_handler() -> impl Responder {
//!         std::thread::sleep(Duration::from_secs(5)); // <-- Bad practice! Will cause the current worker thread to
//! hang!
//!     }
//! ```
//! For this reason, any long, non-cpu-bound operation (e.g. I/O, database operations, etc.) should be expressed as
//! futures or asynchronous functions. Async handlers get executed concurrently by worker threads and thus don’t block
//! execution:
//!
//! ```nocompile
//!     async fn my_handler() -> impl Responder {
//!         tokio::time::sleep(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
//!     }
//! ```
use actix::prelude::*;
use actix_web::{get, http::header::HeaderMap, post, web, web::Data, HttpRequest, HttpResponse, Responder};
use github_pilot_api::{error::GithubProviderError, webhooks::GithubEvent};
use log::*;
use zeroize::Zeroize;

use crate::{
    error::ServerError,
    pub_sub::{GithubEventMessage, PubSubActor},
    utilities::{check_valid_signature, extract_signature, get_secret},
};

type PubSubActorRef = Data<Addr<PubSubActor>>;

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("👍")
}

#[post("/webhook")]
pub async fn github_webhook(
    req: HttpRequest,
    body: web::Bytes,
    pubsub: PubSubActorRef,
) -> Result<HttpResponse, ServerError> {
    let headers = req.headers();
    debug!("💻 Received webhook request");
    let payload = std::str::from_utf8(body.as_ref()).map_err(|e| ServerError::InvalidRequestBody(e.to_string()))?;
    trace!("💻 Decoded payload body. {} bytes", payload.bytes().len());
    if let Err(e) = validate_signature(headers, payload) {
        error!(
            "💻 The webhook signature is invalid. This event will not be processed. {}",
            e
        );
        return Err(ServerError::InvalidSignature);
    }
    let event_name = headers
        .get("x-github-event")
        .ok_or_else(|| ServerError::InvalidEventHeader("x-github-event is missing".into()))?
        .to_str()
        .map_err(|_| ServerError::InvalidEventHeader("x-github-event is not a valid string".into()))?;
    trace!("💻 Extracted event name: {}", event_name);
    match GithubEvent::try_from_webhook_info(event_name, payload) {
        Ok(event) => {
            info!("💻 Github Event Received: [{}], \"{}\"", event_name, event.summary());
            dispatch_event_to_pubsub(pubsub, event_name, event)?;
            Ok(HttpResponse::Ok().finish())
        },
        Err(GithubProviderError::UnknownEvent(s)) => {
            info!(
                "💻 /webhook handler could not handle an \"{}\" event. Discarding it and moving on.",
                s
            );
            Ok(HttpResponse::Ok().finish())
        },
        Err(GithubProviderError::EventDeserializationError(s)) => {
            warn!(
                "💻 /webhook handler could not deserialize a \"{}\". Turn on TRACE level to get more details and \
                 maybe file a bug report?",
                event_name
            );
            trace!("💻 {}", s);
            trace!("💻 JSON payload:\n{}", payload);
            Ok(HttpResponse::Ok().finish())
        },
        Err(e) => {
            warn!(
                "💻 /webhook handler received an unexpected error: {}. Dropping it like yesterday's news.",
                e.to_string()
            );
            Ok(HttpResponse::Ok().finish())
        },
    }
}

fn validate_signature(headers: &HeaderMap, payload: &str) -> Result<(), ServerError> {
    let signature = extract_signature(headers)?;
    let mut secret = get_secret()?;
    check_valid_signature(secret.as_str(), signature, payload)?;
    secret.zeroize();
    trace!("💻 Received webhook signature check passed");
    Ok(())
}

fn dispatch_event_to_pubsub(pubsub: PubSubActorRef, event_name: &str, event: GithubEvent) -> Result<(), ServerError> {
    let msg = GithubEventMessage::new(event_name, event);
    trace!("💻 Dispatching {} to pubsub", event_name);
    match pubsub.try_send(msg) {
        Err(SendError::Full(_)) => {
            warn!("💻 PubSub message queue is full");
            Err(ServerError::MailboxFull)
        },
        Err(SendError::Closed(_)) => {
            warn!("💻 PubSub message queue is closed");
            Err(ServerError::MailboxClosed)
        },
        Ok(()) => {
            trace!("💻 Github event message was dispatched ok.");
            Ok(())
        },
    }
}
