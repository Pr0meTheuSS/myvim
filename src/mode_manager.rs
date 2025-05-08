use crossterm::event::KeyCode;
use crate::state_machine::FSM;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
/// Editor states.
pub enum State {
    NormalMode,
    InsertMode,
    VisualMode,
    CommandLineMode,
}

pub struct ModeManager {
    state_machine: FSM<State, KeyCode>,
    current_state: State,
}

impl ModeManager {
    pub fn new() -> Self {
        use State::*;
        use KeyCode::*;

        let mut state_machine = FSM::<State, KeyCode>::new();

        state_machine.add_transition(InsertMode, Esc, NormalMode);
        state_machine.add_transition(VisualMode, Esc, NormalMode);
        state_machine.add_transition(CommandLineMode, Esc, NormalMode);

        state_machine.add_transition(NormalMode, Char('v'), VisualMode);

        state_machine.add_transition(NormalMode, Char('i'), InsertMode);

        state_machine.add_transition(NormalMode, Char(':'), CommandLineMode);

        Self {
            state_machine,
            current_state: NormalMode,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        if let Some(next) = self.state_machine.next_state(&self.current_state, &key) {
            self.current_state = next;
        }
    }

    pub fn current_state(&self) -> State {
        self.current_state.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_in_normal_mode() {
        let state_manager = ModeManager::new();
        assert_eq!(state_manager.current_state(), State::NormalMode);
    }

    #[test]
    fn test_go_to_insert_mode() {
        let mut state_manager = ModeManager::new();
        state_manager.handle_key(KeyCode::Char('i'));
        assert_eq!(state_manager.current_state(), State::InsertMode);
    }
}
