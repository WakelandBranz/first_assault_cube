use memlib::MemoryWriteExt;

// __int64 newplayerent(void)

// https://github.com/assaultcube/AC/blob/da5cb69c009b4c8fafbb2498787bd4b05d0274e7/source/src/entity.h#L59
const NUM_GUNS: u32 = 9;

#[derive(Debug)]
pub enum Gun {
    Knife = 0,
    Pistol,
    Carbine,
    Shotgun,
    Subgun,
    Sniper,
    Assault,
    Grenade,
    Akimbo,
}

const PLAYER: u64 = 0x5F0E10;

// void __fastcall pickupeffects(int a1, struct_player *player)

// https://github.com/assaultcube/AC/blob/da5cb69c009b4c8fafbb2498787bd4b05d0274e7/source/src/entity.h#L254-L259
const HEALTH: u64 = 0x100;
const ARMOR: u64 = 0x104;
const PRIMARY: u64 = 0x108;
const NEXT_PRIMARY: u64 = 0x10C;
const AKIMBO: u64 = 0x114;

const AMMO_ARRAY: u64 = 0x13C;
const MAG_ARRAY: u64 = 0x118;
const GRENADE: u64 = 0x158;
const GUN_INFO: u64 = 0x15C;