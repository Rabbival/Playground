#[derive(Debug, Clone, Copy, Default)]
pub enum AffectingTimerSetPolicy {
    #[default]
    AlwaysTakeNew,
    IgnoreNewIfAssigned,
}
