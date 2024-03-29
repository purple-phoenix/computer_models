use crate::state_machine::state_machine::State::{S0, S1, S2};
use crate::state_machine::state_machine::Alphabet::{ZERO, ONE};

use std::fmt;
use crate::state_machine::state_machine::AutomatonAcceptance::{Accept, Reject};

#[derive(Debug, PartialOrd, PartialEq)]
enum AutomatonAcceptance {
    Accept,
    Reject
}

impl fmt::Display for AutomatonAcceptance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AutomatonAcceptance::Accept => write!(f, "Accept"),
            AutomatonAcceptance::Reject => write!(f, "Reject")
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum State {
    S0,
    S1,
    S2
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::S0 => write!(f, "S0"),
            State::S1 => write!(f, "S1"),
            State::S2 => write!(f, "S2")
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Alphabet {
    ONE,
    ZERO,
}
impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Alphabet::ONE => write!(f, "1"),
            Alphabet::ZERO => write!(f, "0")
        }
    }
}


fn fsm(stream: &[Alphabet]) -> AutomatonAcceptance {
    return fsm_aux(stream, State::S0)
}

fn fsm_aux(stream: &[Alphabet], state: State) -> AutomatonAcceptance {
    if stream.is_empty() {
        return determine_state_acceptance(state)
    } else {
        let car = &stream[0];
        let cdr = &stream[1..];
        println!("Applying {} to state {}", car, state);

        return match (car, state) {
            (ONE, S0) => fsm_aux(cdr, S1),
            (ZERO, S0) => fsm_aux(cdr, S0),
            (ONE, S1) => fsm_aux(cdr, S0),
            (ZERO, S1) => fsm_aux(cdr, S2),
            (ONE, S2) => fsm_aux(cdr, S2),
            (ZERO, S2) => fsm_aux(cdr, S1),
        }
    }
}

fn abs_state_fun(stream: &[Alphabet],
                 zero_match_fun: fn(&[Alphabet]) -> AutomatonAcceptance,
                 one_match_fun: fn(&[Alphabet]) -> AutomatonAcceptance,
                 is_accept_state: bool) -> AutomatonAcceptance {
    if stream.is_empty() {
        if is_accept_state {
            return Accept
        }
        else {
            return Reject
        }
    }
    else {
        let car = &stream[0];
        let cdr = &stream[1..];
        return match car {
            ZERO => zero_match_fun(cdr),
            ONE => one_match_fun(cdr)
        }
    }
}

fn s0_fun(stream: &[Alphabet]) -> AutomatonAcceptance{
    return abs_state_fun(stream, s0_fun, s1_fun, true);
}

fn s1_fun(stream: &[Alphabet]) -> AutomatonAcceptance {
    return abs_state_fun(stream, s2_fun, s0_fun, false);
}

fn s2_fun(stream: &[Alphabet]) -> AutomatonAcceptance {
    return abs_state_fun(stream, s1_fun, s2_fun, false);
}

fn fsm_ho(stream: &[Alphabet]) -> AutomatonAcceptance {
    return s0_fun(stream)
}

fn determine_state_acceptance(state: State) -> AutomatonAcceptance {
    println!("End State {}", state);
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
    use crate::state_machine::state_machine::AutomatonAcceptance::{Accept, Reject};

    #[test]
    fn test_determine_state_acceptance() {
        assert_eq!(Accept, determine_state_acceptance(State::S0));
        assert_eq!(Reject, determine_state_acceptance(State::S1));
        assert_eq!(Reject, determine_state_acceptance(State::S2));
    }

    #[test]
    fn test_fsm() {
        assert_eq!(Accept, fsm(&[ZERO, ZERO, ZERO]));
        assert_eq!(Reject, fsm(&[ONE, ZERO, ZERO]));
        assert_eq!(Reject, fsm(&[ONE, ONE, ONE, ZERO, ZERO]));
        assert_eq!(Accept, fsm(&[ONE, ONE, ZERO, ONE, ZERO, ONE, ONE, ZERO, ONE]));
    }

    #[test]
    fn test_fsm_ho() {
        assert_eq!(Accept, fsm_ho(&[ZERO, ZERO, ZERO]));
        assert_eq!(Reject, fsm_ho(&[ONE, ZERO, ZERO]));
        assert_eq!(Reject, fsm_ho(&[ONE, ONE, ONE, ZERO, ZERO]));
        assert_eq!(Accept, fsm_ho(&[ONE, ONE, ZERO, ONE, ZERO, ONE, ONE, ZERO, ONE]));
    }

}