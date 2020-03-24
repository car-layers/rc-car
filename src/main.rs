use rc_control::{car, car::DriveController, RCController};
use rppal::pwm::{Channel, Polarity, Pwm};
use std::{error, thread, time::Duration};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Servo test!");
    let mut servo = Pwm::with_frequency(Channel::Pwm0, 50.0, 0.0, Polarity::Normal, false)?;
    let mut ctrl = RCController::new(&mut servo);

    ctrl.steer(car::STEER_LEFT);
    thread::sleep(Duration::from_millis(500));
    ctrl.steer(car::STEER_RIGHT);
    thread::sleep(Duration::from_millis(500));
    ctrl.steer(car::STEER_CENTER);
    thread::sleep(Duration::from_millis(500));

    Ok(())
}
