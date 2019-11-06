#[derive(Debug)]
enum AutomatonAcceptance {
    Accept,
    Reject
}

#[derive(Debug)]
enum State {
    S0,
    S1,
    S2
}

fn fsm(str: String) -> AutomatonAcceptance {
    return fsm_aux(str, State::S0)
}

fn fsm_aux(str: String, state: State) -> AutomatonAcceptance {

}