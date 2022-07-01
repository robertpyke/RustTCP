use std::{env, io};

use std::io::Read;

extern crate tun;
fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "full");

    let mut config = tun::Configuration::default();
    config
        .address((10, 0, 0, 1))
        .netmask((255, 255, 255, 0))
        .up();

    #[cfg(target_os = "linux")]
    config.platform(|config| {
        config.packet_information(true);
    });

    let mut dev = tun::create(&config).unwrap();
    let mut buf = [0; 4096];

    let amount = dev.read(&mut buf).unwrap();
    println!("{:?}", &buf[0..amount]);

    Ok(())
}
