#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerMovementEvent {
    MoveForward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerRotationEvent {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAttackEvent {
    LeftPunch,
    RightPunch,
}
