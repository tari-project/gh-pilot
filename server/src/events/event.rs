/// An enumeration of all the internal broadcast events that can occur in Github Pilot. Clients can subscribe and
/// react to these events by defining an [`EventRule`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Event {
    ReviewsNeeded,
    ReviewsThresholdReached,
    AcksNeeded,
    AcksThresholdReached,
    ChangesRequested,
    #[default]
    Default,
}
