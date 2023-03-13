#[repr(isize)]
pub enum Permissions {
    None = 0,
    Plugin = 1,
    RobloxPlace = 2,
    LocalUser = 3,
    WritePlayer = 4,
    RobloxScript = 5,
    RobloxEngine = 6,
    NoAccessible = 7,
}
