use crate::process::*;
use crate::utils;

// Base pointer
const PLAYER: u64 = 0x0017E0A8;

// Position offsets
const POSITION_X: u64 = 0x2C;
const POSITION_Y: u64 = 0x30;
const POSITION_Z: u64 = 0x28;

// Head position offsets
const HEAD_POSITION_X: u64 = 0x4;
const HEAD_POSITION_Y: u64 = 0xC;
const HEAD_POSITION_Z: u64 = 0x8;

// Camera angles
const CAMERA_X: u64 = 0x34;
const CAMERA_Y: u64 = 0x38;

// Ammo offsets
const ASSAULT_RIFLE_AMMO: u64 = 0x140;
const SMG_AMMO: u64 = 0x138;
const SNIPER_AMMO: u64 = 0x13C;
const SHOTGUN_AMMO: u64 = 0x134;
const PISTOL_AMMO: u64 = 0x12C;
const GRENADE_AMMO: u64 = 0x144;

// Fast fire offsets
const FAST_FIRE_AR: u64 = 0x164;
const FAST_FIRE_SNIPER: u64 = 0x160;
const FAST_FIRE_SHOTGUN: u64 = 0x158;

// Player state offsets
const AUTO_SHOOT: u64 = 0x204;
const HEALTH: u64 = 0xEC;
const ARMOR: u64 = 0xF0;
const PLAYER_NAME: u64 = 0x205;

// Unsure if these work
const PRIMARY: u64 = 0x108;
const NEXT_PRIMARY: u64 = 0x10C;
const AKIMBO: u64 = 0x114;
const AMMO_ARRAY: u64 = 0x13C;
const MAG_ARRAY: u64 = 0x118;
const GRENADE: u64 = 0x158;
const GUN_INFO: u64 = 0x15C;

#[derive(Debug)]
pub struct Player {
    client: Process,
    address: u64,
}

impl Player {
    pub fn new(client: Process) -> Self {
        let address = client.read::<u64>(client.base_address as u64 + PLAYER).unwrap();
        Self { client, address }
    }

    pub fn position_x(&self) -> f32 {
        self.client.read::<f32>(self.address + POSITION_X).unwrap_or(0.0)
    }

    pub fn position_y(&self) -> f32 {
        self.client.read::<f32>(self.address + POSITION_Y).unwrap_or(0.0)
    }

    pub fn position_z(&self) -> f32 {
        self.client.read::<f32>(self.address + POSITION_Z).unwrap_or(0.0)
    }

    pub fn head_position_x(&self) -> f32 {
        self.client.read::<f32>(self.address + HEAD_POSITION_X).unwrap_or(0.0)
    }

    pub fn head_position_y(&self) -> f32 {
        self.client.read::<f32>(self.address + HEAD_POSITION_Y).unwrap_or(0.0)
    }

    pub fn head_position_z(&self) -> f32 {
        self.client.read::<f32>(self.address + HEAD_POSITION_Z).unwrap_or(0.0)
    }

    pub fn assault_rifle_ammo(&self) -> u32 {
        self.client.read::<u32>(self.address + ASSAULT_RIFLE_AMMO).unwrap_or(0)
    }

    pub fn smg_ammo(&self) -> u32 {
        self.client.read::<u32>(self.address + SMG_AMMO).unwrap_or(0)
    }

    pub fn sniper_ammo(&self) -> u32 {
        self.client.read::<u32>(self.address + SNIPER_AMMO).unwrap_or(0)
    }

    pub fn shotgun_ammo(&self) -> u32 {
        self.client.read::<u32>(self.address + SHOTGUN_AMMO).unwrap_or(0)
    }

    pub fn pistol_ammo(&self) -> u32 {
        self.client.read::<u32>(self.address + PISTOL_AMMO).unwrap_or(0)
    }

    pub fn grenade_ammo(&self) -> u32 {
        self.client.read::<u32>(self.address + GRENADE_AMMO).unwrap_or(0)
    }

    pub fn fast_fire_ar(&self) -> bool {
        self.client.read::<u8>(self.address + FAST_FIRE_AR).unwrap_or(0) != 0
    }

    pub fn fast_fire_sniper(&self) -> bool {
        self.client.read::<u8>(self.address + FAST_FIRE_SNIPER).unwrap_or(0) != 0
    }

    pub fn fast_fire_shotgun(&self) -> bool {
        self.client.read::<u8>(self.address + FAST_FIRE_SHOTGUN).unwrap_or(0) != 0
    }

    pub fn health(&self) -> u16 {
        self.client.read::<u16>(self.address + HEALTH).unwrap_or(0)
    }

    pub fn armor(&self) -> u16 {
        self.client.read::<u16>(self.address + ARMOR).unwrap_or(0)
    }

    pub fn auto_shoot(&self) -> bool {
        self.client.read::<u8>(self.address + AUTO_SHOOT).unwrap_or(0) != 0
    }

    pub fn player_name(&self) -> String {
        let name_bytes = self.client.read::<[u8; 16]>(self.address + PLAYER_NAME).unwrap_or([0; 16]);
        String::from_utf8_lossy(&name_bytes)
            .trim_matches(char::from(0))
            .to_string()
    }

    pub fn camera_x(&self) -> f32 {
        self.client.read::<f32>(self.address + CAMERA_X).unwrap_or(0.0)
    }

    pub fn camera_y(&self) -> f32 {
        self.client.read::<f32>(self.address + CAMERA_Y).unwrap_or(0.0)
    }

    ///// SETTERS -------------------------------------------------------------------------

    // Position setters
    pub fn set_position_x(&self, value: f32) -> Option<()> {
        self.client.write(self.address as u64 + POSITION_X, value)
    }

    pub fn set_position_y(&self, value: f32) -> Option<()> {
        self.client.write(self.address as u64 + POSITION_Y, value)
    }

    pub fn set_position_z(&self, value: f32) -> Option<()> {
        self.client.write(self.address as u64 + POSITION_Z, value)
    }

    // Head position setters
    pub fn set_head_position_x(&self, value: f32) -> Option<()> {
        self.client.write(self.address as u64 + HEAD_POSITION_X, value)
    }

    pub fn set_head_position_y(&self, value: f32) -> Option<()> {
        self.client.write(self.address as u64 + HEAD_POSITION_Y, value)
    }

    pub fn set_head_position_z(&self, value: f32) -> Option<()> {
        self.client.write(self.address as u64 + HEAD_POSITION_Z, value)
    }

    // Ammo setters
    pub fn set_assault_rifle_ammo(&self, value: u32) -> Option<()> {
        self.client.write(self.address as u64 + ASSAULT_RIFLE_AMMO, value)
    }

    pub fn set_smg_ammo(&self, value: u32) -> Option<()> {
        self.client.write(self.address as u64 + SMG_AMMO, value)
    }

    pub fn set_sniper_ammo(&self, value: u32) -> Option<()> {
        self.client.write(self.address as u64 + SNIPER_AMMO, value)
    }

    pub fn set_shotgun_ammo(&self, value: u32) -> Option<()> {
        self.client.write(self.address as u64 + SHOTGUN_AMMO, value)
    }

    pub fn set_pistol_ammo(&self, value: u32) -> Option<()> {
        self.client.write(self.address as u64 + PISTOL_AMMO, value)
    }

    pub fn set_grenade_ammo(&self, value: u32) -> Option<()> {
        self.client.write(self.address as u64 + GRENADE_AMMO, value)
    }

    // Fast fire setters
    pub fn set_fast_fire_ar(&self, enabled: bool) -> Option<()> {
        self.client.write(self.address as u64 + FAST_FIRE_AR, enabled as u8)
    }

    pub fn set_fast_fire_sniper(&self, enabled: bool) -> Option<()> {
        self.client.write(self.address as u64 + FAST_FIRE_SNIPER, enabled as u8)
    }

    pub fn set_fast_fire_shotgun(&self, enabled: bool) -> Option<()> {
        self.client.write(self.address as u64 + FAST_FIRE_SHOTGUN, enabled as u8)
    }

    // Player stats setters
    pub fn set_health(&self, value: u16) -> Option<()> {
        self.client.write(self.address as u64 + HEALTH, value)
    }

    pub fn set_armor(&self, value: u16) -> Option<()> {
        self.client.write(self.address as u64 + ARMOR, value)
    }

    pub fn set_auto_shoot(&self, enabled: bool) -> Option<()> {
        self.client.write(self.address as u64 + AUTO_SHOOT, enabled as u8)
    }

    pub fn set_player_name(&self, name: &str) -> Option<()> {
        // Create a fixed-size buffer of 16 bytes
        let mut buffer = [0u8; 16];
        // Copy the name bytes into the buffer, truncating if too long
        let name_bytes = name.as_bytes();
        let copy_len = name_bytes.len().min(15); // Leave room for null terminator
        buffer[..copy_len].copy_from_slice(&name_bytes[..copy_len]);
        // Write the entire buffer
        self.client.write(self.address as u64 + PLAYER_NAME, buffer)
    }

    pub fn set_random_name(&self) -> Option<()> {
        let random_name = utils::random_name_ascii();
        self.set_player_name(&random_name)
    }

    // Camera angle setters
    pub fn set_camera_x(&self, value: f32) -> Option<()> {
        self.client.write(self.address as u64 + CAMERA_X, value)
    }

    pub fn set_camera_y(&self, value: f32) -> Option<()> {
        self.client.write(self.address as u64 + CAMERA_Y, value)
    }

    // Convenience methods for setting multiple values
    pub fn set_position(&self, x: f32, y: f32, z: f32) -> Option<()> {
        self.set_position_x(x)?;
        self.set_position_y(y)?;
        self.set_position_z(z)
    }

    pub fn set_head_position(&self, x: f32, y: f32, z: f32) -> Option<()> {
        self.set_head_position_x(x)?;
        self.set_head_position_y(y)?;
        self.set_head_position_z(z)
    }

    pub fn set_camera(&self, x: f32, y: f32) -> Option<()> {
        self.set_camera_x(x)?;
        self.set_camera_y(y)
    }
}