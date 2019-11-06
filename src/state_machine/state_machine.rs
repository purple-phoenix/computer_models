#[derive(Debug, PartialOrd, PartialEq)]
enum AutomatonAcceptance {
    Accept,
    Reject
}

#[derive(Debug, PartialOrd, PartialEq)]
enum State {
    S0,
    S1,
    S2
}

fn fsm(str: String) -> AutomatonAcceptance {
    return fsm_aux(str, State::S0)
}

fn fsm_aux(str: String, state: State) -> AutomatonAcceptance {
    return AutomatonAcceptance::Reject
}

fn determine_state_acceptance(state: State) -> AutomatonAcceptance {
    if state == State::S0 {
        return AutomatonAcceptance::Accept
    }
    else {
        return AutomatonAcceptance::Reject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_state_acceptance() {
        assert_eq!(AutomatonAcceptance::Accept, determine_state_acceptance(State::S0));
        assert_eq!(AutomatonAcceptance::Reject, determine_state_acceptance(State::S1));
        assert_eq!(AutomatonAcceptance::Reject, determine_state_acceptance(State::S2));
    }
}