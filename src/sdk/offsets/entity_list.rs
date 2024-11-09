pub(crate) const ENTITY_LIST: u32 = 0x18AC04;
pub(crate) const ENTITY_LIST_SIZE: u32 = ENTITY_LIST + 0x8;
pub(crate) const MAX_PLAYERS: u32 = 32;

// Thank you https://github.com/scannells/ac_rhack/blob/main/src/player/mod.rs
// AssaultCube has a custom vector that holds pointers to enemies,
// which are also Players. We use this struct to read the enemy player's positions
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct AcVector {
    pub player_addresses: u32,   // pointer to the buffer of pointers to the enemies
    pub capacity: i32,           // max size of the buffer
    pub elements: i32            // how many elements there actually are
}

impl Default for AcVector {
    fn default() -> Self {
        AcVector {
            player_addresses: 0,
            capacity: 32,
            elements: 0,
        }
    }
}
