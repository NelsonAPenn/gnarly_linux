use gnarly_linux::{install, Configuration, User};

fn main() {
    let configuration = Configuration
    {
        boot_partition_bytes: 5_000_000,
        root_password: "My FDE password".into(),
        users: vec![
            User{
                 full_name: "Joe Schmo (I use Arch BTW)".into(),
                username: "joeschmo".into(),
                sudoer: true,
                password: "my password".into(),
                autologin: true
            }
        ],
    };

    install(&configuration);
}
