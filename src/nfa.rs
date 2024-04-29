use crate::regex_ast::RegexAst;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

type RefState = Rc<RefCell<State>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    id: usize,
    accepting: bool,
    transitions: Vec<Transition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    symbol: Vec<char>,
    next_state: RefState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NFA {
    start: RefState,
}

impl State {
    pub fn new(accepting: bool) -> RefState {
        static ID: AtomicUsize = AtomicUsize::new(0);
        Rc::new(RefCell::new(State {
            id: ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            transitions: Vec::new(),
            accepting,
        }))
    }

    pub fn add_trans(&mut self, symbol: Vec<char>, next_state: RefState) {
        self.transitions.push(Transition { symbol, next_state });
    }
}

impl NFA {
    pub fn new(start: RefState) -> NFA {
        NFA { start }
    }

    fn epsilon_closure(&self, current_states: &mut Vec<RefState>) {
        let mut i = 0;

        while i < current_states.len() {
            let mut ep_transitions: Vec<RefState> = Vec::new();
            for tran in &current_states[i].borrow().transitions {
                if tran.symbol.is_empty() && !current_states.contains(&tran.next_state) {
                    ep_transitions.push(tran.next_state.clone());
                }
            }
            current_states.extend(ep_transitions);
            i += 1;
        }
    }

    fn step(&self, current_states: &mut Vec<RefState>, c: char) {
        let mut next_states: Vec<RefState> = Vec::new();
        for state in current_states.iter() {
            for tran in &state.borrow().transitions {
                if tran.symbol.contains(&c) {
                    next_states.push(tran.next_state.clone());
                }
            }
        }
        current_states.clear();
        current_states.extend(next_states);
    }

    pub fn run(&self, input: &str) -> bool {
        let mut current_states = vec![self.start.clone()];
        let mut chars = input.chars();

        while let Some(c) = chars.next() {
            // Epsilon Closure
            self.epsilon_closure(&mut current_states);
            // Move states
            self.step(&mut current_states, c);
        }
        self.epsilon_closure(&mut current_states);
        current_states.iter().any(|state| state.borrow().accepting)
    }
}

fn Convert(re: RegexAst) -> (RefState, RefState) {
    todo!();
}

pub fn test() {
    // create simple NFA

    let state1 = State::new(false);
    let state2 = State::new(false);
    let state3 = State::new(true);

    state1.borrow_mut().add_trans(vec!['a'], state2.clone());
    state2.borrow_mut().add_trans(vec![], state3.clone());
    state3.borrow_mut().add_trans(vec![], state2.clone());

    let nfa = NFA::new(state1);
    let x = nfa.run("a");
    println!("{}", x);
}
