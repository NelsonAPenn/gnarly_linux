/*
 * Copyright 2025 Nelson Penn.
 *
 * This file is part of The Gnarly Installer.
 *
 * The Gnarly Installer is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by the Free
 * Software Foundation, either version 3 of the License, or (at your option) any
 * later version.
 *
 * The Gnarly Installer is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License along with
 * The Gnarly Installer. If not, see <https://www.gnu.org/licenses/>.
 */

use gnarly_linux::{install, Configuration, User};

fn main() {
    let configuration = Configuration {
        target_disk: "/dev/nvme0n1".into(),
        boot_partition_bytes: 5_000_000,
        root_password: "My FDE password".into(),
        users: vec![User {
            full_name: "Joe Schmo (I use Arch BTW)".into(),
            username: "joeschmo".into(),
            sudoer: true,
            password: "my password".into(),
            autologin: true,
        }],
    };

    install(&configuration);
}
