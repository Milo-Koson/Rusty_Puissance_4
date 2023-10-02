
//pub use self::PowerEvent;

/**
 * Destinators events. 
 */

pub fn here() {
    println!("Power event !");
}

pub(crate) struct PowerEvent {
    pub(crate) event_type: EventType,
    pub(crate) position: KEY,
}

pub enum KEY {
    A,
    B,
    C,
    D,
    E,
    F,
    G
}

pub union EventType {
    to_state_manager: std::mem::ManuallyDrop<STATE_MANAGER_EVENT>,
    to_displayer: std::mem::ManuallyDrop<DISPLAYER_EVENT>,
    to_grid_manager: std::mem::ManuallyDrop<GRID_MANAGER_EVENT>,
    to_action_taker: std::mem::ManuallyDrop<ACTION_TAKER_EVENT>
}

pub enum STATE_MANAGER_EVENT {
    GIVING_POSITION,
    END
}

pub enum DISPLAYER_EVENT {
    END
}

pub enum GRID_MANAGER_EVENT {
    LAUNCH,
    END
}

pub enum ACTION_TAKER_EVENT {
    END
}

pub enum CHRONO_EVENT {
    STOP_CRHONO,
    END
}
