#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum PlayerState {
    NotInGame,
    Living,
    Dead,
}
