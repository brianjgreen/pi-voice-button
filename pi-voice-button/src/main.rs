use rppal::gpio::{Gpio, Level};
use std::{thread, time};

fn main() {
    // Button
    let gpio23 = Gpio::new()
        .expect("GPIO23 no good")
        .get(23)
        .expect("GPIO23 in use");

    // LED
    let mut gpio25 = Gpio::new()
        .expect("GPIO25 no good")
        .get(25)
        .expect("GPIO25 in use")
        .into_output();

    loop {
        let button_state = gpio23.read();
        println!("23={}", button_state,);
        if button_state == Level::Low {
            gpio25.set_high();
        } else {
            gpio25.set_low();
        }
        thread::sleep(time::Duration::from_millis(100));
    }
}
