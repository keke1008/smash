#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerMovementEvent {
    MoveForward,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerRotationEvent {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerAttackEvent {
    LeftPunch,
    RightPunch,
}
