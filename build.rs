#![deny(warnings)]

use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET")?;

    if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=armv6m");
    } else if target.starts_with("thumbv7m-") {
        println!("cargo:rustc-cfg=armv7m");
    } else if target.starts_with("thumbv7em-") {
        println!("cargo:rustc-cfg=armv7m");
    } else if target.starts_with("armv7r-") {
        println!("cargo:rustc-cfg=armv7r");
    } else if target.starts_with("thumbv8m.base") {
        println!("cargo:rustc-cfg=armv8m_base");
    } else if target.starts_with("thumbv8m.main") {
        println!("cargo:rustc-cfg=armv8m_main");
    }

    Ok(())
}
