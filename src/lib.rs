

pub struct User
{
    pub full_name: String,
    pub username: String,
    pub sudoer: bool,
    pub password: String,
    pub autologin: bool,
}


pub struct Configuration
{
    /// Soft recommendation for boot partition size. Defaulted to a reasonable value.
    /// Actual partition size may vary for alignment reasons.
    pub boot_partition_bytes: u64,

    /// Root password
    /// 
    /// Also the FDE password
    pub root_password: String, 
    pub users: Vec<User>,
}

pub fn install(config: &Configuration)
{
    todo!()
}
