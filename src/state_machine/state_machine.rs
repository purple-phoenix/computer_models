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

fn fsm(str: &str) -> AutomatonAcceptance {
    return fsm_aux(str, State::S0)
}

fn fsm_aux(str: &str, state: State) -> AutomatonAcceptance {
    if str.is_empty() {
        return determine_state_acceptance(state)
    }
    else {
        let bit = str.chars().next().unwrap();
        return match state {
             S0 => AutomatonAcceptance::Reject,
             S1 => AutomatonAcceptance::Reject,
             S2 => AutomatonAcceptance::Reject
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

fn car_cadr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
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

    #[test]
    fn test_car_cadr() {
        let some_str = "some_str";
        let empty_str = "";
        assert_eq!(("s", "ome_str"), car_cadr(some_str));
        assert_eq!(("", ""), car_cadr(empty_str));
    }
}