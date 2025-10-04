use rust_gpiozero::*;
use std::{
    sync::mpsc,
    thread::{self, *},
    time::{self, Duration},
};

fn main() {
    println!("Hello, world!");

    let (sender, reciever) = mpsc::channel::<f32>();
    let x = sender.clone();
    let y = sender.clone();

    let tire1_handle = thread::spawn(move || tire(4, x));
    let tire2_handle = thread::spawn(move || tire(4, y));

    'main: loop {}
}

fn tire(pin: u8, channel: mpsc::Sender<f32>) -> Result<()> {
    let mut dig = DigitalInputDevice::new(pin);

    'fewf: loop {
        let now = time::Instant::now();
        dig.wait_for_active(None);
        if false {
            break 'fewf;
        } else {
            if let Err(goo) = channel.send(6.7) {
                break 'fewf;
            } else {
                println!("Tire on pin {} moved", pin);
                dig.wait_for_inactive(None);
                let elapsed = now.elapsed();
                println!("Elapsed: {:?}", elapsed);
                println!("Frequency: {}", 1.0 / elapsed.as_secs_f32());
                channel.send(1.0 / elapsed.as_secs_f32()).unwrap();
            }
        }
    }

    Ok(())
}
