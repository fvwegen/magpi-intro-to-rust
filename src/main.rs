extern crate sensehat;
use sensehat::{Colour, SenseHat, SenseHatError};
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), SenseHatError> {
    let mut hat = SenseHat::new()?;
    hat.clear()?;
    let temp = hat.get_temperature_from_humidity()?;
    println!("It's {} in here", temp);
    println!("Shake me!");
    loop {
        let accel = hat.get_accel_raw()?;
        println!("{:?}", accel);
        if [accel.x, accel.y, accel.z].iter().any(|g| *g > 1.5) {
            hat.text("Earthquake!", Colour::WHITE, Colour::RED)?;
            sleep(Duration::from_millis(1000));
            hat.clear()?;
        }
        sleep(Duration::from_millis(250));
    }
}
