use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

type RefState = Rc<RefCell<State>>;

static ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    id: usize,
    accepting: bool,
    transitions: Vec<Transition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    symbol: Option<char>,
    next_state: RefState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NFA {
    start: RefState,
}

impl State {
    pub fn new(accepting: bool) -> RefState {
        Rc::new(RefCell::new(State {
            transitions: Vec::new(),
            accepting,
            id: ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }))
    }

    pub fn add_transition(&mut self, symbol: Option<char>, next_state: RefState) {
        self.transitions.push(Transition { symbol, next_state });
    }
}

impl NFA {
    pub fn new(start: RefState) -> NFA {
        NFA { start }
    }
}

pub fn run(nfa: &NFA, input: &str) -> bool {
    let mut current_states = vec![nfa.start.clone()];
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        let mut i = 0;

        // Epsilon Closure
        while i < current_states.len() {
            let mut ep_transitions: Vec<RefState> = Vec::new();
            for tran in &current_states[i].borrow().transitions {
                if tran.symbol == None && !current_states.contains(&tran.next_state) {
                    ep_transitions.push(tran.next_state.clone());
                }
            }
            current_states.extend(ep_transitions);
            i += 1;
        }

        // Move states
        let mut next_states: Vec<RefState> = Vec::new();
        for state in &current_states {
            for tran in &state.borrow().transitions {
                if tran.symbol == Some(c) {
                    next_states.push(tran.next_state.clone());
                }
            }
        }
        current_states = next_states;
    }

    current_states.iter().any(|state| state.borrow().accepting)
}

pub fn test() {
    // create simple NFA

    let state1 = State::new(false);
    let state2 = State::new(true);
    let state3 = State::new(true);

    state1
        .borrow_mut()
        .add_transition(Some('a'), state2.clone());
    state2.borrow_mut().add_transition(None, state3.clone());
    state3
        .borrow_mut()
        .add_transition(Some('b'), state2.clone());
    state2
        .borrow_mut()
        .add_transition(Some('x'), state3.clone());

    let nfa = NFA::new(state1);
    let x = run(&nfa, "az");
    println!("{}", x);
}
