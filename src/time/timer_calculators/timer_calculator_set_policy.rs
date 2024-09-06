#[derive(Debug, Clone, Copy, Default)]
pub enum TimerCalculatorSetPolicy {
    #[default]
    AlwaysTakeNew,
    IgnoreNewIfAssigned,
}
