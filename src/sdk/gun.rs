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

