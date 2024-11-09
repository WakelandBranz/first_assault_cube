use crate::process::Process;
use crate::sdk::player::Player;
use super::offsets::{entity_list::*, player::HEALTH};

use log::{info, debug};
use crate::sdk::player;

#[derive(Clone, Debug)]
pub struct EntityList {
    mem: Process,
    pub address: u32,
    pub entity_count: u32,
    pub entities: Vec<Player>,
}

impl EntityList {
    pub fn new(mem: Process) -> Self {
        let address = mem.read::<u32>(mem.base_address as u32 + ENTITY_LIST).unwrap();
        let entity_count = mem.read::<u32>(mem.base_address as u32 + ENTITY_LIST_SIZE).unwrap();
        Self {
            mem,
            address,
            entity_count,
            entities: Vec::with_capacity(MAX_PLAYERS as usize),
        }
    }

    /// Use before updating entity list!
    fn update_entity_count(&mut self) {
        // Read the entity list size
        match self.mem.read::<u32>(self.mem.base_address as u32 + ENTITY_LIST_SIZE) {
            Some(size) => self.entity_count = size,
            None => self.entity_count = 0
        }
    }

    /// Update entity list, run before reading from entity vector!
    pub fn update_entities(&mut self) {
        self.update_entity_count();

        self.entities = Vec::with_capacity(self.entity_count as usize);

        for i in 0..self.entity_count {
            let player_addr = match self.mem.read::<u32>(self.address + (i * 0x4)) {
                Some(addr) => addr,
                None => continue
            };

            // Sometimes players are null
            if player_addr == 0x0 {
                continue
            }

            // Double check to ensure that the entity list is valid.
            let player: Player = match self.mem.read::<Player>(player_addr) {
                Some(player) => player,
                None => {
                    debug!("Entity list likely invalid (could be loading). Resetting...");
                    self.entities = Vec::with_capacity(MAX_PLAYERS as usize);
                    break
                }
            };

            //debug!("Entity {} position: {}", i, player.pos);
            self.entities.push(player)
        }
    }
}