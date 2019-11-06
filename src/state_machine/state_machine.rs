use crate::state_machine::state_machine::State::{S0, S1, S2};

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

#[derive(Debug, PartialOrd, PartialEq)]
enum Alphabet {
    ONE,
    ZERO
}

fn fsm(stream: &[Alphabet]) -> AutomatonAcceptance {
    return fsm_aux(stream, State::S0)
}

fn fsm_aux(stream: &[Alphabet], state: State) -> AutomatonAcceptance {
    if stream.is_empty() {
        return determine_state_acceptance(state)
    }
    else {
        let car = &stream[0];
        let cdr = &stream[1..];
        return match (car, state) {
            (ONE, S0) => AutomatonAcceptance::Reject,
            (ZERO, S0) => AutomatonAcceptance::Reject,
            (ONE, S1) => AutomatonAcceptance::Reject,
            (ZERO, S1) => AutomatonAcceptance::Reject,
            (ONE, S2) => AutomatonAcceptance::Reject,
            (ZERO, S2) => AutomatonAcceptance::Reject,
        }
    }
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
    use crate::state_machine::state_machine::Alphabet::{ZERO, ONE};

    #[test]
    fn test_determine_state_acceptance() {
        assert_eq!(AutomatonAcceptance::Accept, determine_state_acceptance(State::S0));
        assert_eq!(AutomatonAcceptance::Reject, determine_state_acceptance(State::S1));
        assert_eq!(AutomatonAcceptance::Reject, determine_state_acceptance(State::S2));
    }

}