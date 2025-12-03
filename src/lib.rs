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

pub struct User {
    pub full_name: String,
    pub username: String,
    pub sudoer: bool,
    pub password: String,
    pub autologin: bool,
}

pub struct Configuration {
    /// Disk which is the installation target, e.g. "/dev/nvme0n1"
    pub target_disk: String,

    /// Soft recommendation for boot partition size. Defaulted to a reasonable value.
    /// Actual partition size may vary for alignment reasons.
    pub boot_partition_bytes: u64,

    /// Root password
    ///
    /// Also the FDE password
    pub root_password: String,
    pub users: Vec<User>,
}

pub fn install(config: &Configuration) {
    todo!()
}
