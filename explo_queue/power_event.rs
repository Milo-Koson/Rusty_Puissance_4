
/**
 * Destinators events. 
 */

pub fn here() {
    println!("Power event !");
}

pub struct PowerEvent {
    event_type: EventType,
    position: KEY,
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
    to_state_manager: STATE_MANAGER_EVENT,
    to_displayer: DISPLAYER_EVENT,
    to_grid_manager: GRID_MANAGER,
    to_action_taker: ACTION_TAKER_EVENT
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
