use std::error::Error;
use std::net::IpAddr;

use jumpingsumo_rs::JumpingSumo;

fn main() -> Result<(), Box<dyn Error>> {
    let drone_address = "192.168.1.1".parse::<IpAddr>().unwrap();

    let sumo = JumpingSumo::new(drone_address)?;

    sumo.forward()?;

    Ok(())
}
