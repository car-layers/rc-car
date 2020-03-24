use embedded_hal as hal;
use std::{error, fmt, ops::Range};

pub mod car;
use car::DriveController;

pub struct RCController<'a> {
    servo: &'a mut dyn hal::PwmPin<Duty = f64>,
    last: car::DriveCommand,
    range: Range<f64>,
}

impl<'a> RCController<'a> {
    const DUTY_RANGE: Range<f64> = (0.05..0.1);

    pub fn new<T>(servo: &'a mut T) -> Self
    where
        T: hal::PwmPin<Duty = f64>,
    {
        servo.enable();
        let mut ctrl = Self {
            servo,
            last: Default::default(),
            range: Self::DUTY_RANGE,
        };
        ctrl.steer(car::STEER_CENTER);
        ctrl
    }
}

impl DriveController for RCController<'_> {}

impl car::Commander<car::DriveCommand> for RCController<'_> {
    fn command(&mut self, state: car::DriveCommand) {
        let duty = normalize(state.steer, &car::STEERING_RANGE, &self.range);
        self.servo.set_duty(duty);
        self.last = state;
    }

    fn last_command(&self) -> car::DriveCommand {
        self.last
    }
}

fn normalize(val: f32, from_range: &Range<f32>, to_range: &Range<f64>) -> f64 {
    ((val - from_range.start) as f64 * (to_range.end - to_range.start))
        / (from_range.end - from_range.start) as f64
        + to_range.start
}

#[derive(Debug)]
pub struct RCError {}

impl fmt::Display for RCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oupsi, an error!")
    }
}

impl error::Error for RCError {}
