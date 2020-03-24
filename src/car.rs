use std::ops::Range;

pub trait Commander<T: Copy> {
    fn command(&mut self, state: T);
    fn last_command(&self) -> T;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct DriveCommand {
    pub steer: f32,
    pub throttle: f32,
    pub brake: f32,
    pub reverse: bool,
}

pub const STEERING_RANGE: Range<f32> = (-1.0..1.0);
pub const STEER_LEFT: f32 = -1.0;
pub const STEER_CENTER: f32 = 0.0;
pub const STEER_RIGHT: f32 = 1.0;

pub trait DriveController: Commander<DriveCommand> {
    fn steer(&mut self, steer: f32) {
        self.command(DriveCommand {
            steer,
            ..self.last_command()
        })
    }

    fn throttle(&mut self, throttle: f32, reverse: bool) {
        self.command(DriveCommand {
            throttle,
            reverse,
            ..self.last_command()
        })
    }

    fn brake(&mut self, brake: f32) {
        self.command(DriveCommand {
            brake,
            ..self.last_command()
        })
    }
}
