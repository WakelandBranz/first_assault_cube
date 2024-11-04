use std::mem::offset_of;
use crate::sdk::{Vector2, Vector3};
use crate::sdk::weapon::{Weapon, Ammo};
use crate::process::Process;
use log::{debug, error};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Player {
    // Virtual table pointer (first 4 bytes in a class with virtual functions)
    vtable: u32,
    pub pos_head: Vector3,    // 0x0004
    pub velocity: Vector3,    // 0x0010
    _pad_001c: [u8; 12], // 0x001C
    pub pos: Vector3,        // 0x0028
    pub camera: Vector2,     // 0x0034
    _pad_003c: [u8; 176], // 0x003C
    pub health: u32,         // 0x00EC
    pub armor: u32,          // 0x00F0
    _pad_00f4: [u8; 273], // 0x00F4
    pub name: [u8; 16],      // 0x0205
    _pad_0215: [u8; 339], // 0x0214
    pub weapon_ptr: u32, // 0x036C
}

#[derive(Clone)]
pub struct PlayerManager {
    mem: Process,
    address: u32,
    player: Player,
}

impl PlayerManager {
    pub fn new(mem: crate::process::Process, address: u32) -> Option<Self> {
        let player = mem.read::<Player>(address)?;
        Some(Self {
            mem,
            address,
            player,
        })
    }

    // Update the player data from memory
    pub fn update(&mut self) -> Option<()> {
        self.player = self.mem.read::<Player>(self.address)?;
        Some(())
    }

    // Position (Head) methods
    pub fn head_position(&self) -> Vector3 {
        self.player.pos_head
    }

    pub fn set_head_position(&mut self, pos: Vector3) -> Option<()> {
        let offset = memoffset::offset_of!(Player, pos_head) as u32;
        self.mem.write(self.address + offset, pos)?;
        self.player.pos_head = pos;
        Some(())
    }

    // Velocity methods
    pub fn velocity(&self) -> Vector3 {
        self.player.velocity
    }

    pub fn set_velocity(&mut self, vel: Vector3) -> Option<()> {
        let offset = memoffset::offset_of!(Player, velocity) as u32;
        self.mem.write(self.address + offset, vel)?;
        self.player.velocity = vel;
        Some(())
    }

    // Position methods
    pub fn position(&self) -> Vector3 {
        self.player.pos
    }

    pub fn set_position(&mut self, pos: Vector3) -> Option<()> {
        let offset = memoffset::offset_of!(Player, pos) as u32;
        self.mem.write(self.address + offset, pos)?;
        self.player.pos = pos;
        Some(())
    }

    // Camera methods
    pub fn camera(&self) -> Vector2 {
        self.player.camera
    }

    pub fn set_camera(&mut self, cam: Vector2) -> Option<()> {
        let offset = memoffset::offset_of!(Player, camera) as u32;
        self.mem.write(self.address + offset, cam)?;
        self.player.camera = cam;
        Some(())
    }

    // Health methods
    pub fn health(&self) -> u32 {
        self.player.health
    }

    pub fn set_health(&mut self, health: u32) -> Option<()> {
        let offset = memoffset::offset_of!(Player, health) as u32;
        self.mem.write(self.address + offset, health)?;
        self.player.health = health;
        Some(())
    }

    // Armor methods
    pub fn armor(&self) -> u32 {
        self.player.armor
    }

    pub fn set_armor(&mut self, armor: u32) -> Option<()> {
        let offset = memoffset::offset_of!(Player, armor) as u32;
        self.mem.write(self.address + offset, armor)?;
        self.player.armor = armor;
        Some(())
    }

    // Name methods
    pub fn name(&self) -> String {
        String::from_utf8_lossy(&self.player.name).trim_matches('\0').to_string()
    }

    pub fn set_name(&mut self, name: &str) -> Option<()> {
        let offset = memoffset::offset_of!(Player, name) as u32;
        let mut name_bytes = [0u8; 16];
        let bytes = name.as_bytes();
        let len = bytes.len().min(15); // Ensure we don't overflow
        name_bytes[..len].copy_from_slice(&bytes[..len]);
        self.mem.write(self.address + offset, name_bytes)?;
        self.player.name = name_bytes;
        Some(())
    }

    // Weapon pointer methods
    fn weapon_ptr(&self) -> u32 {
        self.player.weapon_ptr
    }

    // Helper methods for weapon access

    /// Returns the Ammo struct
    fn get_weapon(&self) -> Weapon {
        self.mem.read::<Weapon>(self.player.weapon_ptr)
            .unwrap_or_else(|| panic!("Couldn't retrieve weapon struct in get_weapon!"))
    }

    /// Returns the Ammo struct
    fn get_ammo(&self) -> Ammo {
        self.mem.read::<Ammo>(self.get_weapon().ammo_ptr)
            .unwrap_or_else(|| panic!("Couldn't retrieve ammo struct in get_ammo!"))
    }

    pub fn ammo(&self) -> u32 {
        self.get_ammo().current
    }

    pub fn set_ammo(&self, value: u32) -> Option<()> {
        let weapon = self.get_weapon();
        let ammo_current_address = weapon.ammo_ptr + memoffset::offset_of!(Ammo, current) as u32;

        match self.mem.write(ammo_current_address, value) {
            Some(_) => Some(()),
            None => {
                error!("Failed to set ammo value to {}", value);
                None
            }
        }
    }

    pub fn weapon_usage_count(&self) -> u32 {
        let weapon = self.get_weapon();
        let ammo = self.mem.read::<Ammo>(weapon.ammo_ptr).unwrap_or_else(|| panic!("Could not get ammo struct in player::ammo!"));
        ammo.usage_count
    }

    // Raw address access
    pub fn address(&self) -> u32 {
        self.address
    }

    // Get base player struct
    pub fn player(&self) -> &Player {
        &self.player
    }

    // Check if player is alive
    pub fn is_alive(&self) -> bool {
        self.player.health > 0
    }

    // Vector helper methods
    pub fn distance_to(&self, other: &PlayerManager) -> f32 {
        let dx = self.player.pos.x - other.player.pos.x;
        let dy = self.player.pos.y - other.player.pos.y;
        let dz = self.player.pos.z - other.player.pos.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn distance_to_pos(&self, pos: &Vector3) -> f32 {
        let dx = self.player.pos.x - pos.x;
        let dy = self.player.pos.y - pos.y;
        let dz = self.player.pos.z - pos.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
