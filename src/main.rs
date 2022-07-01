use std::io;

use std::io::Read;

extern crate tun;

const IP_ADDRESS: (u8, u8, u8, u8) = (10, 0, 0, 1);
const MASK: (u8, u8, u8, u8) = (255, 255, 255, 0);

fn main() -> io::Result<()> {
    let mut config = tun::Configuration::default();
    config.address(IP_ADDRESS).netmask(MASK).up();

    #[cfg(target_os = "linux")]
    config.platform(|config| {
        config.packet_information(true);
    });

    let mut dev = tun::create(&config).unwrap();
    let mut buf = [0; 4096];

    loop {
        let amount = dev.read(&mut buf)?;

        // First 4 bytes of read data are the Address family.
        // 2 for AF_INET,
        // 1E for AF_INET6, etc..
        let address_family = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);

        println!("buf: {:?}", &buf[..amount]);
        println!("address_family: {:#x}", address_family);

        // e.g. buf: [0, 0, 0, 2, 69, 0, 0, 84, 58, 9, 0, 0, 64, 1, 44, 159, 10, 0, 0, 1, 10, 0, 0, 1, 8, 0, 9, 254, 156, 39, 0, 1, 98, 190, 175, 32, 0, 6, 84, 241, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55]

        // 0..4 = Address Family
        // [0, 0, 0, 2]
        // 2 = AF_INET

        // 5 => version + IHL
        // 0x69 ==> 0100 0101 (in binary)
        //
        // 0b0100
        // version = 4 == ipv4
        //
        // 0b0101
        // Internet Header Length (IHL) = 5
        // IHL - number of 32-bit words in the header (min is 5 = 20 bytes)

        let p = etherparse::Ipv4HeaderSlice::from_slice(&buf[4..amount]);

        match p {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {
                println!("header_slice: {:?}", value);
                println!("version: {:?}", value.version());
                println!("ihl: {:?}", value.ihl());
                println!("source_addr: {:?}", value.source_addr());
                println!("destination_addr: {:?}", value.destination_addr());
                println!("identification: {:?}", value.identification());
                println!("protocol: {:?}", value.protocol());
                println!("ttl: {:?}", value.ttl());
            }
        }
        println!("----------------")
    }
}
