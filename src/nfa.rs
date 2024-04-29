use std::cell::RefCell;
use std::rc::Rc;

type RefState = Rc<RefCell<State>>;

#[derive(Debug)]
pub struct State {
    transitions: Vec<Transition>,
    accepting: bool,
}

#[derive(Debug)]
pub struct Transition {
    symbol: char,
    next_state: RefState,
}

#[derive(Debug)]
pub struct NFA {
    start: RefState,
}

impl State {
    pub fn new(accepting: bool) -> State {
        State {
            transitions: Vec::new(),
            accepting,
        }
    }

    pub fn add_transition(&mut self, symbol: char, next_state: RefState) {
        self.transitions.push(Transition { symbol, next_state });
    }
}

impl Transition {
    pub fn new(symbol: char, next_state: RefState) -> Transition {
        Transition { symbol, next_state }
    }
}

impl NFA {
    pub fn new(start: RefState) -> NFA {
        NFA { start }
    }
}

pub fn test() {
    // create simple NFA

    let state1 = Rc::new(RefCell::new(State::new(false)));
    let state2 = Rc::new(RefCell::new(State::new(true)));

    state1.borrow_mut().add_transition('a', state2.clone());
    state2.borrow_mut().add_transition('b', state1.clone());

    let nfa = NFA::new(state1);
    dbg!(nfa);
}
