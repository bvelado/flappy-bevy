#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum AppState {
    Launching(LaunchingState),
    InGame(InGameState),
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum LaunchingState {
    Loading,
    Ready,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum InGameState {
    Playing,
}
