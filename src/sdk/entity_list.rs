use crate::process::Process;
use crate::sdk::player::Player;
use super::offsets::entity_list::*;

use log::{info, debug};
use crate::sdk::player;

#[derive(Clone, Debug)]
pub struct EntityList {
    mem: Process,
    pub address: u32,
    pub entities: Vec<Player>,
}

impl EntityList {
    pub fn new(mem: Process) -> Self {
        let address = mem.read::<u32>(mem.base_address as u32 + ENTITY_LIST).unwrap();
        Self {
            mem,
            address,
            entities: Vec::new(),
        }
    }

    /// Returns a vector of all other players in the lobby
    pub fn update_entities(&mut self) {
        let entities_base = self.mem.base_address as u32 + ENTITY_LIST;
        info!("Entities base: 0x{:x}", entities_base);

        // Read the AcVector structure from the target process
        let vec_of_entities = match self.mem.read::<AcVector>(entities_base) {
            Some(vec) => {
                info!("Vector read: elements={}, addresses_base=0x{:x}",
                  vec.elements, vec.player_addresses);
                vec
            },
            None => {
                debug!("Failed to read AcVector at 0x{:x}", entities_base);
                self.entities = Vec::new();
                return;
            }
        };

        let mut entities = Vec::with_capacity(32);

        // Fill in the vector of enemies
        for i in 0..vec_of_entities.elements {
            let current_address = vec_of_entities.player_addresses + (i * 4) as u32;
            debug!("Reading player at offset 0x{:x}", current_address);

            let player_addr = match self.mem.read::<u32>(current_address) {
                Some(addr) => addr as u64,
                None => {
                    debug!("Failed to read player address at 0x{:x}", current_address);
                    continue;
                },
            };

            // sometimes pointers are NULL
            if player_addr == 0x0 {
                debug!("Null player pointer at index {}", i);
                continue;
            }

            let player = Player::new(self.mem.clone(), player_addr as u32);
            debug!("Player {} health: {}", i, player.health());
            entities.push(player);
        }

        info!("Found {} valid players", entities.len());
        self.entities = entities;
    }



    /*pub fn update_entities(&mut self) -> bool {
        let mut player_vector: Vec<Player> = Vec::new();
        let mut current_offset: u32 = 0x4;  // 32-bit alignment in target process

        loop {
            let current_address = self.address + current_offset;
            debug!("Reading entity at offset 0x{:x} (address: 0x{:x})", current_offset, current_address);

            // Read 32-bit pointers from target process
            match self.mem.read::<u32>(current_address) {
                Some(player_address) => {
                    if player_address == 0 {
                        break;
                    }
                    debug!("Found player at: 0x{:x}", player_address);
                    let player = Player::new(self.mem.clone(), player_address);
                    info!("Player name: {}", player.name());
                    player_vector.push(player);
                    current_offset += 0x4;  // Move by 4 bytes since target is 32-bit
                }
                None => break
            }

            if current_offset > 0x1000 {
                debug!("Hit safety limit for entity list");
                break;
            }
        }

        if player_vector.is_empty() {
            debug!("Unsuccessfully attempted to grab entity list.");
            return false;
        }

        self.entities = player_vector;
        true
    }*/

    /*pub fn entities(&self) -> Option<Vec<Player>> {
        let mut player_vector: Vec<Player> = Vec::new();
        let mut current_offset = 0x4; // Add offset counter, first entity is empty so skip 0x0

        // Get list of entities
        loop {
            // Read the player address and match on the Result
            match self.mem.read::<u64>(self.address + current_offset) {
                Some(player_address) => {
                    let player = Player::new(self.mem.clone(), player_address);
                    info!("Player name: {}", player.name());
                    player_vector.push(player);
                    current_offset += 0x4;
                }
                None =>  {
                    break
                }
            }
        }

        if player_vector.is_empty() {
            debug!("Unsuccessfully attempted to grab entity list.");
            return None;
        }

        Some(player_vector)
    }*/
}