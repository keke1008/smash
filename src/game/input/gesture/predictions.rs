#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) enum MovementPrediction {
    #[default]
    Stay,
    Run,
    LeftPunch,
    RightPunch,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) enum RotationPrediction {
    #[default]
    Stay,
    RotateLeft,
    RotateRight,
}
