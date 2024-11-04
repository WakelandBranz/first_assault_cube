use std::any::Any;
use Weapon;
use crate::process::*;
use crate::sdk::{Vector2, Vector3};
use crate::utils;
use super::offsets::player::*;
use super::weapon::*;

#[derive(Clone, Debug)]
pub struct Player {
    mem: Process,
    pub address: u32,
}

impl Player {
    pub fn new(mem: Process, offset: u32) -> Self {
        let address = mem.read::<u32>(offset).unwrap();
        Self { mem, address }
    }

    pub fn position_x(&self) -> f32 {
        self.mem.read::<f32>(self.address + POSITION_X).unwrap_or(0.0)
    }

    pub fn position_y(&self) -> f32 {
        self.mem.read::<f32>(self.address + POSITION_Y).unwrap_or(0.0)
    }

    pub fn position_z(&self) -> f32 {
        self.mem.read::<f32>(self.address + POSITION_Z).unwrap_or(0.0)
    }

    pub fn position(&self) -> Vector3 {
        let x = self.position_x();
        let y = self.position_y();
        let z = self.position_z();
        Vector3 { x, y, z, }
    }

    pub fn head_position_x(&self) -> f32 {
        self.mem.read::<f32>(self.address + HEAD_POSITION_X).unwrap_or(0.0)
    }

    pub fn head_position_y(&self) -> f32 {
        self.mem.read::<f32>(self.address + HEAD_POSITION_Y).unwrap_or(0.0)
    }

    pub fn head_position_z(&self) -> f32 {
        self.mem.read::<f32>(self.address + HEAD_POSITION_Z).unwrap_or(0.0)
    }

    pub fn head_position(&self) -> Vector3 {
        let x = self.head_position_x();
        let y = self.head_position_y();
        let z = self.head_position_z();
        Vector3 { x, y, z}
    }

    pub fn assault_rifle_ammo(&self) -> u32 {
        self.mem.read::<u32>(self.address + ASSAULT_RIFLE_AMMO).unwrap_or(0)
    }

    pub fn smg_ammo(&self) -> u32 {
        self.mem.read::<u32>(self.address + SMG_AMMO).unwrap_or(0)
    }

    pub fn sniper_ammo(&self) -> u32 {
        self.mem.read::<u32>(self.address + SNIPER_AMMO).unwrap_or(0)
    }

    pub fn shotgun_ammo(&self) -> u32 {
        self.mem.read::<u32>(self.address + SHOTGUN_AMMO).unwrap_or(0)
    }

    pub fn pistol_ammo(&self) -> u32 {
        self.mem.read::<u32>(self.address + PISTOL_AMMO).unwrap_or(0)
    }

    pub fn grenade_ammo(&self) -> u32 {
        self.mem.read::<u32>(self.address + GRENADE_AMMO).unwrap_or(0)
    }

    pub fn fast_fire_ar(&self) -> bool {
        self.mem.read::<u8>(self.address + FAST_FIRE_AR).unwrap_or(0) != 0
    }

    pub fn fast_fire_sniper(&self) -> bool {
        self.mem.read::<u8>(self.address + FAST_FIRE_SNIPER).unwrap_or(0) != 0
    }

    pub fn fast_fire_shotgun(&self) -> bool {
        self.mem.read::<u8>(self.address + FAST_FIRE_SHOTGUN).unwrap_or(0) != 0
    }

    pub fn health(&self) -> u16 {
        log::debug!("unsuccessful");
        self.mem.read::<u16>(self.address + HEALTH).unwrap_or(0)
    }

    pub fn armor(&self) -> u16 {
        self.mem.read::<u16>(self.address + ARMOR).unwrap_or(0)
    }

    pub fn auto_shoot(&self) -> bool {
        self.mem.read::<u8>(self.address + AUTO_SHOOT).unwrap_or(0) != 0
    }

    pub fn primary_gun(&self) -> Weapon {
        let value = self.mem.read::<u32>(self.address + PRIMARY).unwrap();
        assert!(value < NUM_GUNS);

        match value {
            0 => Weapon::Knife,
            1 => Weapon::Pistol,
            2 => Weapon::Carbine,
            3 => Weapon::Shotgun,
            4 => Weapon::Subgun,
            5 => Weapon::Sniper,
            6 => Weapon::Assault,
            7 => Weapon::Grenade,
            8 => Weapon::Akimbo,
            _ => unreachable!(),
        }
    }

    pub fn name(&self) -> String {
        let name_bytes = self.mem.read::<[u8; 16]>(self.address + PLAYER_NAME).unwrap_or([0; 16]);
        String::from_utf8_lossy(&name_bytes)
            .trim_matches(char::from(0))
            .to_string()
    }

    pub fn camera_x(&self) -> f32 {
        self.mem.read::<f32>(self.address + CAMERA_X).unwrap_or(0.0)
    }

    pub fn camera_y(&self) -> f32 {
        self.mem.read::<f32>(self.address + CAMERA_Y).unwrap_or(0.0)
    }

    pub fn viewport(&self) -> Vector2 {
        let x = self.camera_x();
        let y = self.camera_y();
        Vector2 { x, y }
    }

    ///// SETTERS -------------------------------------------------------------------------

    // Position setters
    pub fn set_position_x(&self, value: f32) -> Option<()> {
        self.mem.write(self.address + POSITION_X, value)
    }

    pub fn set_position_y(&self, value: f32) -> Option<()> {
        self.mem.write(self.address + POSITION_Y, value)
    }

    pub fn set_position_z(&self, value: f32) -> Option<()> {
        self.mem.write(self.address + POSITION_Z, value)
    }

    // Head position setters
    pub fn set_head_position_x(&self, value: f32) -> Option<()> {
        self.mem.write(self.address + HEAD_POSITION_X, value)
    }

    pub fn set_head_position_y(&self, value: f32) -> Option<()> {
        self.mem.write(self.address + HEAD_POSITION_Y, value)
    }

    pub fn set_head_position_z(&self, value: f32) -> Option<()> {
        self.mem.write(self.address + HEAD_POSITION_Z, value)
    }

    // Ammo setters
    pub fn set_assault_rifle_ammo(&self, value: u32) -> Option<()> {
        self.mem.write(self.address + ASSAULT_RIFLE_AMMO, value)
    }

    pub fn set_smg_ammo(&self, value: u32) -> Option<()> {
        self.mem.write(self.address + SMG_AMMO, value)
    }

    pub fn set_sniper_ammo(&self, value: u32) -> Option<()> {
        self.mem.write(self.address + SNIPER_AMMO, value)
    }

    pub fn set_shotgun_ammo(&self, value: u32) -> Option<()> {
        self.mem.write(self.address + SHOTGUN_AMMO, value)
    }

    pub fn set_pistol_ammo(&self, value: u32) -> Option<()> {
        self.mem.write(self.address + PISTOL_AMMO, value)
    }

    pub fn set_grenade_ammo(&self, value: u32) -> Option<()> {
        self.mem.write(self.address + GRENADE_AMMO, value)
    }

    // Fast fire setters
    pub fn set_fast_fire_ar(&self, enabled: bool) -> Option<()> {
        self.mem.write(self.address + FAST_FIRE_AR, enabled as u8)
    }

    pub fn set_fast_fire_sniper(&self, enabled: bool) -> Option<()> {
        self.mem.write(self.address + FAST_FIRE_SNIPER, enabled as u8)
    }

    pub fn set_fast_fire_shotgun(&self, enabled: bool) -> Option<()> {
        self.mem.write(self.address + FAST_FIRE_SHOTGUN, enabled as u8)
    }

    // Player stats setters
    pub fn set_health(&self, value: u16) -> Option<()> {
        self.mem.write(self.address + HEALTH, value)
    }

    pub fn set_armor(&self, value: u16) -> Option<()> {
        self.mem.write(self.address + ARMOR, value)
    }

    pub fn set_auto_shoot(&self, enabled: bool) -> Option<()> {
        self.mem.write(self.address + AUTO_SHOOT, enabled as u8)
    }

    pub fn set_name(&self, name: &str) -> Option<()> {
        // Create a fixed-size buffer of 16 bytes
        let mut buffer = [0u8; 16];
        // Copy the name bytes into the buffer, truncating if too long
        let name_bytes = name.as_bytes();
        let copy_len = name_bytes.len().min(15); // Leave room for null terminator
        buffer[..copy_len].copy_from_slice(&name_bytes[..copy_len]);
        // Write the entire buffer
        self.mem.write(self.address + PLAYER_NAME, buffer)
    }

    pub fn set_random_name(&self) -> Option<()> {
        let random_name = utils::random_name_ascii();
        self.set_name(&random_name)
    }

    // Camera angle setters
    pub fn set_camera_yaw(&self, value: f32) -> Option<()> {
        self.mem.write(self.address + CAMERA_X, value)
    }

    pub fn set_camera_pitch(&self, value: f32) -> Option<()> {
        self.mem.write(self.address + CAMERA_Y, value)
    }

    // Convenience methods for setting multiple values
    pub fn set_position(&self, x: f32, y: f32, z: f32) -> Option<()> {
        self.set_position_x(x)?;
        self.set_position_y(y)?;
        self.set_position_z(z)
    }

    pub fn set_head_position(&self, position: Vector3) -> Option<()> {
        self.set_head_position_x(position.x)?;
        self.set_head_position_y(position.y)?;
        self.set_head_position_z(position.z)
    }

    pub fn set_camera(&self, yaw: f32, pitch: f32) -> Option<()> {
        self.set_camera_yaw(yaw)?;
        self.set_camera_pitch(pitch)
    }
}