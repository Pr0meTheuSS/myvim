use std::collections::HashMap;
use std::hash::Hash;

pub struct FSM<State, Event> {
    transitions: HashMap<(State, Event), State>,
}

impl<State, Event> FSM<State, Event>
where
    State: Eq + Hash + Clone,
    Event: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
        }
    }

    pub fn add_transition(&mut self, from: State, event: Event, to: State) {
        self.transitions.insert((from, event), to);
    }

    pub fn next_state(&self, current: &State, event: &Event) -> Option<State> {
        self.transitions.get(&(current.clone(), event.clone())).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_empty_state() {
        let fsm = FSM::<String, String>::new();
        let state = String::from("state");
        let event = String::from("event");
        assert_eq!(fsm.next_state(&state, &event), None);
    }

    #[test]
    fn test_get_existing_state() {
        let mut fsm = FSM::<String, String>::new();
        let state1 = String::from("state1");
        let state2 = String::from("state2");
        let event = String::from("event");
        fsm.add_transition(state1.clone(), event.clone(), state2.clone());
        assert_eq!(fsm.next_state(&state1, &event), Some(state2));
    }
}
