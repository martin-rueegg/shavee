mod args;
mod logic;

use logic::{create_zfs_file, create_zfs_yubi, unlock_zfs_pass};
use logic::{print_mode_file, print_mode_yubi, unlock_zfs_file, unlock_zfs_yubi};
use args::Sargs;
use sha2::{Digest, Sha512};
use shavee_lib::zfs::*;
use std::process::exit;

fn main() {
    let args = Sargs::new();
    let pass = rpassword::prompt_password_stderr("Dataset Password: ");
    let mut pass = match pass {
        Ok(pass) => pass,
        Err(error) => {
            eprintln!("Error: Failed to read Password");
            eprintln!("Error: {}", error);
            exit(1)
        }
    };
    pass.push_str("Aveesha");

    match args.umode.as_str() {
        "yubikey" => match args.mode.as_str() {
            "print" => print_mode_yubi(pass, args.yslot),
            "pam" | "mount" => unlock_zfs_yubi(pass, args.dataset, args.yslot),
            "create" => create_zfs_yubi(pass, args.dataset, args.yslot),
            _ => unreachable!(),
        },
        "file" => match args.mode.as_str() {
            "print" => print_mode_file(pass, &args.file, args.port),
            "pam" | "mount" => unlock_zfs_file(pass, args.file, args.dataset, args.port),
            "create" => create_zfs_file(pass, args.file, args.dataset, args.port),
            _ => unreachable!(),
        },
        "password" => {
            let key = format!("{:x}", Sha512::digest(pass.as_bytes()));
            match args.mode.as_str() {
                "print" => println!("{}", key),
                "pam" | "mount" => unlock_zfs_pass(key, args.dataset),
                "create" => match zfs_create(key, args.dataset) {
                    Ok(()) => (),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        exit(1)
                    }
                },
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
