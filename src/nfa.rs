use colored::Colorize;

use crate::regex_ast::RegexAst;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

type RefState = Rc<RefCell<State>>;

type LoopMap = HashMap<usize, InputMap>;
type InputMap = HashMap<Input, Vec<Loop>>;
type Input = Vec<char>;
type Loop = Vec<usize>;

pub fn find_loops(
    start: RefState,
    current: RefState,
    input_map: &mut InputMap,
    current_loop: Loop,
    current_input: Input,
) -> InputMap {
    for tran in &current.borrow().transitions {
        if current_loop.contains(&tran.id) {
            continue;
        }

        let mut new_loop = current_loop.clone();
        new_loop.push(tran.id);

        if tran.next_state == start {
            if tran.symbol.is_empty() {
                input_map
                    .entry(current_input.clone())
                    .or_insert(Vec::new())
                    .push(new_loop.clone());
            }
            for sym in &tran.symbol {
                let mut new_input = current_input.clone();
                new_input.push(*sym);
                input_map
                    .entry(new_input.clone())
                    .or_insert(Vec::new())
                    .push(new_loop.clone());
            }
        }

        // Epsilon transition
        if tran.symbol.is_empty() {
            find_loops(
                start.clone(),
                tran.next_state.clone(),
                input_map,
                new_loop.clone(),
                current_input.clone(),
            );
        }

        for sym in &tran.symbol {
            let mut new_input = current_input.clone();
            new_input.push(*sym);
            find_loops(
                start.clone(),
                tran.next_state.clone(),
                input_map,
                new_loop.clone(),
                new_input,
            );
        }
    }

    return input_map.clone();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    id: usize,
    accepting: bool,
    transitions: Vec<Transition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    id: usize,
    symbol: Vec<char>,
    next_state: RefState,
}

#[derive(Clone, PartialEq, Eq)]
pub struct NFA {
    pub start: RefState,
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

    pub fn add_transition(&mut self, symbol: Vec<char>, next_state: &RefState) {
        static ID: AtomicUsize = AtomicUsize::new(0);
        self.transitions.push(Transition {
            id: ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            symbol,
            next_state: next_state.clone(),
        });
    }
}

impl NFA {
    pub fn new(start: RefState) -> NFA {
        NFA { start }
    }

    pub fn get_states(&self) -> Vec<RefState> {
        let mut states = vec![self.start.clone()];
        let mut i = 0;
        while i < states.len() {
            let current_state = states[i].clone();
            for tran in &current_state.borrow().transitions {
                if !states.contains(&tran.next_state) {
                    states.push(tran.next_state.clone());
                }
            }
            i += 1;
        }
        return states.clone();
    }

    pub fn all_loops(&self) -> LoopMap {
        let mut loop_map = HashMap::new();
        let states = self.get_states();
        for state in &states {
            loop_map.insert(
                state.borrow().id,
                find_loops(
                    state.clone(),
                    state.clone(),
                    &mut HashMap::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            );
        }
        loop_map
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

impl fmt::Display for NFA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(self.start.clone());
        while let Some(state) = queue.pop_front() {
            if visited.contains(&state.borrow().id) {
                continue;
            }
            visited.insert(state.borrow().id);
            for tran in &state.borrow().transitions {
                write!(
                    f,
                    "{} -- {:?} --{}-> {}\n",
                    if state.borrow().accepting {
                        state.borrow().id.to_string().green()
                    } else if state == self.start {
                        state.borrow().id.to_string().red()
                    } else {
                        state.borrow().id.to_string().white()
                    },
                    tran.symbol,
                    tran.id,
                    // tran.next_state.borrow().id
                    if tran.next_state.borrow().accepting {
                        tran.next_state.borrow().id.to_string().green()
                    } else {
                        tran.next_state.borrow().id.to_string().white()
                    }
                )?;
                queue.push_back(tran.next_state.clone());
            }
        }
        Ok(())
    }
}

pub fn create_nfa(re: &RegexAst) -> NFA {
    let (start, end) = convert(re.clone());
    end.borrow_mut().accepting = true;
    NFA::new(start)
}

fn convert(re: RegexAst) -> (RefState, RefState) {
    let start = State::new(false);

    let end = State::new(false);

    match re {
        RegexAst::Epsilon => {
            start.borrow_mut().add_transition(Vec::new(), &end);
            return (start, end);
        }
        RegexAst::Any => {
            let c1: RefState = State::new(false);
            let c2: RefState = State::new(false);
            start.borrow_mut().add_transition(Vec::new(), &c1);
            c1.borrow_mut().add_transition(RegexAst::alphabet(), &c2);
            return (start, c2);
        }
        RegexAst::CharClass(is_complement, char_vec) => {
            let characters = if is_complement {
                RegexAst::alphabet()
                    .into_iter()
                    .filter(|c| !char_vec.contains(c))
                    .collect()
            } else {
                char_vec
            };

            let c1: RefState = State::new(false);
            let c2: RefState = State::new(false);
            start.borrow_mut().add_transition(Vec::new(), &c1);
            c1.borrow_mut().add_transition(characters, &c2);
            return (start, c2);
        }
        RegexAst::Literal(character) => {
            let c1: RefState = State::new(false);
            let c2: RefState = State::new(false);
            start.borrow_mut().add_transition(Vec::new(), &c1);
            c1.borrow_mut().add_transition(vec![character], &c2);
            return (start, c2);
        }
        RegexAst::Concat(reg1, reg2) => {
            let (state11, state12) = convert(*reg1);
            let (state21, state22) = convert(*reg2);
            start.borrow_mut().add_transition(Vec::new(), &state11);
            state12.borrow_mut().add_transition(Vec::new(), &state21);
            state22.borrow_mut().add_transition(Vec::new(), &end);
            return (start, end);
        }
        RegexAst::Or(reg1, reg2) => {
            let (state11, state12) = convert(*reg1);
            let (state21, state22) = convert(*reg2);
            start.borrow_mut().add_transition(Vec::new(), &state11);
            start.borrow_mut().add_transition(Vec::new(), &state21);
            state12.borrow_mut().add_transition(Vec::new(), &end);
            state22.borrow_mut().add_transition(Vec::new(), &end);
            return (start, end);
        }
        RegexAst::Star(reg) => {
            let (state11, state12) = convert(*reg);
            start.borrow_mut().add_transition(Vec::new(), &state11);
            state11.borrow_mut().add_transition(Vec::new(), &state12);
            state12.borrow_mut().add_transition(Vec::new(), &state11);
            state12.borrow_mut().add_transition(Vec::new(), &end);
            return (start, end);
        }
    }
}
