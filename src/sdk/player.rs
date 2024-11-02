use crate::process::*;

const PLAYER: u64 = 0x0017E0A8;

// void __fastcall pickupeffects(int a1, struct_player *player)

// https://github.com/assaultcube/AC/blob/da5cb69c009b4c8fafbb2498787bd4b05d0274e7/source/src/entity.h#L254-L259
const HEALTH: u64 = 0xEC;
const ARMOR: u64 = 0x104;
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
    address: u32,
}

impl Player {
    pub fn new(client: Process) -> Self {
        let address = client.read::<u32>(client.base_address as u64 + PLAYER).unwrap();
        Self { client, address }
    }

    pub fn health(&self) -> u16 {
        self.client.read::<u16>(self.address as u64 + HEALTH).unwrap()
    }
}