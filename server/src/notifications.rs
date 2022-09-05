//! Notifications are a section of rule definitions that define who should be notified when a given rule triggers.
//! In truth they're really specialised `actions`, but are defined separately because they're so commonly used that
//! it's nice to have a separate API for them.
pub struct Notification;
