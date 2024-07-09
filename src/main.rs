use rand::RngCore;
use std::fmt::{self, Display};

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octate = &self.0;

        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octate[0],octate[1],octate[2],
            octate[3],octate[4],octate[5]
        )
    }
}

impl MacAddress{
    fn new() -> MacAddress {
        let mut octates: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octates);
        octates[0] |= 0b_0000_0011;
    
        MacAddress { 0: octates }
    }

    fn is_local(&self) -> bool {
        (self.0[0] & 0b_0000_0010) == 0b_0000_0010
    }

    fn is_unicast(&self) -> bool {
        (self.0[0] & 0b_0000_0001) == 0b_0000_0001
    }
    
}

fn main() {
    let mac = MacAddress::new();
    assert!(mac.is_local());
    assert!(mac.is_unicast());
    println!("mac: {}", mac);
}
