/* This is a small PID controller library written in Rust. It has not been
thoroughly tested, so is probably not ready for use on real hardware at the
moment.

Although this one was written from scratch (and primarily for educational value)
there is another Rust PID control library available at
https://github.com/mbr/pid_control-rs

Please feel free to contact with any questions, comments, or concerns.
*/

// Sets the time-step for behavior checks
// This is also used in the example 'main.rs'
pub const DT: f32 = 0.01;

#[derive(Clone, Copy)]
pub struct Position {pub x: f32}
#[derive(Clone, Copy)]
pub struct PIDSet {pub pre_error: f32, pub integral: f32}
#[derive(Clone, Copy)]
pub struct PIDController {pub input: Position, pub setpoint: Position, kp: f32, ki: f32, kd: f32, rmv: PIDSet}



impl PIDController {

    // Creates a new PID controller
    // Takes 4 arguments: proportional, integral, and derivative gains, plus
    // a set of two 'remembered values' that are used in each iteration of the
    // controller's logic
    pub fn new(kp: f32, ki: f32, kd: f32, rmv:PIDSet) -> Self {
        PIDController {
            input: Position {x: 0.0},
            setpoint: Position {x: 0.0},
            kp: kp,
            ki: ki,
            kd: kd,
            rmv: rmv
        }
    }

    // Resets all PID controller values back to zero
    // TODO: Could be useful for doing error handling, or for gain tuning loops,
    // if the default values for the gains are set to something other than zero
    // when a value exceeds desired limits
    pub fn default() -> Self {
        PIDController {
            input: Position {x: 0.0},
            setpoint: Position {x: 0.0},
            kp: 0.0,
            ki: 0.0,
            kd: 0.0,
            rmv: PIDSet {pre_error: 0.0, integral: 0.0}
        }
    }

    // Primary functionality of the PID loop, takes a state (here, a position)
    // and a desired state (named setpoint)
    pub fn control(&mut self,mut value: Position, setpoint: Position) -> Position {

        let error = setpoint.x - value.x;
        let integral = self.rmv.integral + (error*DT);
        let derivative = (error - self.rmv.pre_error)/DT;
        let output = (self.kp*error) + (self.ki*self.rmv.integral) + (self.kd*derivative);
        value.x = value.x + output;
        self.rmv = PIDSet {pre_error: error, integral: integral};
        value
    }

}

impl PIDSet {

    pub fn new() -> PIDSet {
        PIDSet {pre_error: 0.0, integral: 0.0}
    }

}
