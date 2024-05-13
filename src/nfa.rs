use crate::regex_ast::RegexAst;
use crate::{nfa, regex_ast};
use std::cell::{Ref, RefCell};
use std::os::linux::raw::stat;
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

    pub fn add_trans(&mut self, symbol: Vec<char>, next_state: &RefState) {
        self.transitions.push(Transition {
            symbol,
            next_state: next_state.clone(),
        });
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

pub fn createNfa(re: &RegexAst) -> NFA {
    let (start, mut end) = convert(re.clone());
    end.borrow_mut().accepting = true;
    NFA::new(start)
}

fn convert(re: RegexAst) -> (RefState, RefState) {
    let start = State::new(false);

    let end = State::new(false);

    match re {
        RegexAst::Epsilon => {
            start.borrow_mut().add_trans(Vec::new(), &end);
            return (start, end);
        }
        RegexAst::Any => return (start, end),
        RegexAst::CharClass(boolean_value, char_vec) => {
            let c1: RefState = State::new(false);
            let c2: RefState = State::new(false);
            start.borrow_mut().add_trans(Vec::new(), &c1);
            c1.borrow_mut().add_trans(char_vec, &c2);
            return (start, c2);
        }
        RegexAst::Literal(character) => {
            let c1: RefState = State::new(false);
            let c2: RefState = State::new(false);
            start.borrow_mut().add_trans(Vec::new(), &c1);
            c1.borrow_mut().add_trans(vec![character], &c2);
            return (start, c2);
        }
        RegexAst::Concat(reg1, reg2) => {
            let (state11, state12) = convert(*reg1);
            let (state21, state22) = convert(*reg2);
            start.borrow_mut().add_trans(Vec::new(), &state11);
            state12.borrow_mut().add_trans(Vec::new(), &state21);
            state22.borrow_mut().add_trans(Vec::new(), &end);
            return (start, end);
        }
        RegexAst::Or(reg1, reg2) => {
            let (state11, state12) = convert(*reg1);
            let (state21, state22) = convert(*reg2);
            start.borrow_mut().add_trans(Vec::new(), &state11);
            start.borrow_mut().add_trans(Vec::new(), &state21);
            state12.borrow_mut().add_trans(Vec::new(), &end);
            state22.borrow_mut().add_trans(Vec::new(), &end);
            return (start, end);
        }
        RegexAst::Star(reg) => {
            let (state11, state12) = convert(*reg);
            start.borrow_mut().add_trans(Vec::new(), &state11);
            state11.borrow_mut().add_trans(Vec::new(), &state12);
            state12.borrow_mut().add_trans(Vec::new(), &state11);
            state12.borrow_mut().add_trans(Vec::new(), &end);
            return (start, end);
        }
    }
}
pub fn test() {
    // create simple NFA

    let state1 = State::new(false);
    let state2 = State::new(false);
    let state3 = State::new(true);

    state1.borrow_mut().add_trans(vec!['a'], &state2);
    state2.borrow_mut().add_trans(vec![], &state3);
    state3.borrow_mut().add_trans(vec![], &state2);
    let nfa = NFA::new(state1);
    let x = nfa.run("a");
}
