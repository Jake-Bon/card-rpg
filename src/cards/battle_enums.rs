#[derive(PartialEq, Eq)]
pub enum TurnPhase {
    NotInitialized,
    PreTurnP1,
    TurnP1,
    PostTurnP1,
    PreTurnP2,
    TurnP2,
    PostTurnP2,
    RoundOver,
    BattleOver,
}

#[derive(PartialEq, Eq)]
pub enum BattleOutcome {
    Undetermined,
    VictoryP1,
    VictoryP2,
    Tie,
}
