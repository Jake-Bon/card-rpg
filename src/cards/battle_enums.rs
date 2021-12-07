#[derive(PartialEq, Eq,Debug)]
pub enum TurnPhase {
    NotInitialized,
    NotInitOnlineP1,
    NotInitOnlineP2,
    PreMulliganPhase,
    PreMullOnlineP1,
    PreMullOnlineP2,
    MullOnlineP1,
    MullOnlineP2,
    ConnectionLost,
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

#[derive(PartialEq, Eq, Debug)]
pub enum BattleOutcome {
    Undetermined,
    VictoryP1,
    VictoryP2,
    Tie,
}
