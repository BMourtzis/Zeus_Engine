use std::{collections::BTreeMap, sync::Mutex};

use winit::event::{ElementState, VirtualKeyCode};

//TODO: make InputManager as a trait and implement per window

lazy_static! {
    static ref INPUT: Mutex<InputManager> = Mutex::new(InputManager::init());
}

//Public methods
pub fn update_btn(
    btn: VirtualKeyCode,
    state: ElementState,
) {
    INPUT.lock().unwrap().update_btn(btn, state)
}

pub fn is_btn_down(btn: VirtualKeyCode) -> bool {
    INPUT.lock().unwrap().is_btn_down(btn)
}

struct InputManager {
    btn_map: BTreeMap<VirtualKeyCode, ElementState>,
}

impl InputManager {
    pub fn init() -> InputManager {
        InputManager {
            btn_map: BTreeMap::new(),
        }
    }

    pub fn update_btn(
        &mut self,
        btn: VirtualKeyCode,
        state: ElementState,
    ) {
        if let Some(entry) = self.btn_map.get_mut(&btn) {
            *entry = state;
        } else {
            self.btn_map.insert(btn, state);
        }
    }

    pub fn is_btn_down(
        &self,
        btn: VirtualKeyCode,
    ) -> bool {
        if let Some(state) = self.btn_map.get(&btn) {
            match state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            }
        } else {
            false
        }
    }
}
