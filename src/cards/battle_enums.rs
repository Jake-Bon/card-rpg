#[derive(PartialEq, Eq,Debug)]
pub enum TurnPhase {
    NotInitialized,
    NotInitOnlineP1,
    NotInitOnlineP2,
    PreMulliganPhase,
    MulliganPhase,
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
