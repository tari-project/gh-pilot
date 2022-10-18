use std::fmt::Display;

use actix::Message;
use github_pilot_api::GithubEvent;

use crate::events::Progress;

/// An enumeration of all the internal messaging events that can occur in Github Pilot. These have the same
/// enumeration as [`super::Event`], but include additional information useful for testing against `EventConstraints`.
pub enum BroadcastEvent {
    ReviewsNeeded(Box<Progress>),
    ReviewsThresholdReached,
    AcksNeeded(Box<Progress>),
    AcksThresholdReached,
    ChangesRequested,
}

impl Display for BroadcastEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BroadcastEvent::ReviewsNeeded(p) => write!(f, "ReviewsNeeded: {}/{}", p.current, p.total),
            BroadcastEvent::AcksNeeded(p) => write!(f, "AcksNeeded: {}/{}", p.current, p.total),
            BroadcastEvent::ReviewsThresholdReached => write!(f, "ReviewsThresholdReached"),
            BroadcastEvent::AcksThresholdReached => write!(f, "AcksThresholdReached"),
            BroadcastEvent::ChangesRequested => write!(f, "ChangesRequested"),
        }
    }
}

pub struct BroadcastEventMessage {
    pub event: BroadcastEvent,
    pub github_event: Option<GithubEvent>,
}

impl BroadcastEventMessage {
    pub fn new(event: BroadcastEvent, github_event: Option<GithubEvent>) -> Self {
        BroadcastEventMessage { event, github_event }
    }
}

impl Message for BroadcastEventMessage {
    type Result = ();
}
