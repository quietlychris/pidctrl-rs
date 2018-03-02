/* This is a small control systems library written in Rust. It has not been
thoroughly tested, so is probably not ready for use on real hardware at the
moment.

Although this one was written from scratch (and primarily for educational value)
there is another Rust PID control library available at
https://github.com/mbr/pid_control-rs

In addition, it can be used implement a spring-mass-damper type control
logic to a system, where the second-order derivative is being used to control
the actual position (i.e. using a force input to control position, based
on distance from desired position and inertial mass of the system).

Please feel free to contact with any questions, comments, or concerns.
*/

// Sets the time-step for behavior checks
// This is also used in the example 'main.rs'
pub const DT: f32 = 0.01;

#[derive(Clone, Copy)]
pub struct Position {pub x: f32}
#[derive(Clone, Copy)]
pub struct Velocity {pub x: f32}
#[derive(Clone, Copy)]
pub struct Acceleration {pub x: f32}
#[derive(Clone, Copy)]
pub struct State {pub position: Position, pub velocity: Velocity, pub acceleration: Acceleration}

#[derive(Clone, Copy)]
pub struct PIDSet {pub pre_error: f32, pub integral: f32}
#[derive(Clone, Copy)]
pub struct PIDController {pub input: Position, pub setpoint: Position, kp: f32, ki: f32, kd: f32, rmv: PIDSet}
#[derive(Clone, Copy)]
pub struct SMD {pub error: f32, m: f32, c: f32, k: f32} // spring-mass-damper


impl PIDController {


    pub fn new(kp: f32, ki: f32, kd: f32, rmv:PIDSet) -> Self {

        // Creates a new PID controller
        // Takes 4 arguments: proportional, integral, and derivative gains, plus
        // a set of two 'remembered values' that are used in each iteration of the
        // controller's logic

        PIDController {
            input: Position {x: 0.0},
            setpoint: Position {x: 0.0},
            kp: kp,
            ki: ki,
            kd: kd,
            rmv: rmv
        }
    }


    pub fn default() -> Self {

        // Resets all PID controller values back to zero
        // TODO: Could be useful for doing error handling, or for gain tuning loops,
        // if the default values for the gains are set to something other than zero
        // when a value exceeds desired limits

        PIDController {
            input: Position {x: 0.0},
            setpoint: Position {x: 0.0},
            kp: 0.0,
            ki: 0.0,
            kd: 0.0,
            rmv: PIDSet {pre_error: 0.0, integral: 0.0}
        }
    }


    pub fn control(&mut self,mut value: Position, setpoint: Position) -> Position {

        // Primary functionality of the PID loop, takes a state (here, a position)
        // and a desired state (named setpoint)

        let error = setpoint.x - value.x;
        let integral = self.rmv.integral + (error*DT);
        let derivative = (error - self.rmv.pre_error)/DT;
        let output = (self.kp*error) + (self.ki*self.rmv.integral) + (self.kd*derivative);
        value.x = value.x + output;
        self.rmv = PIDSet {pre_error: error, integral: integral};
        value
    }

}

impl SMD {

    pub fn default() -> Self {
        SMD {error: 0.0, m: 0.0, c: 0.0, k: 0.0}
    }

    pub fn new(m: f32, c: f32, k:f32 ) -> Self {
        SMD {error: 0.0, m: m, c: c, k: k}
    }

}

impl PIDSet {

    pub fn new(pre_error: f32, integral: f32) -> Self {
        PIDSet {pre_error: pre_error, integral: integral}
    }

    pub fn default() -> Self {
        PIDSet {pre_error: 0.0, integral: 0.0}
    }

}

pub fn find_error(a: f32, b: f32) -> f32 {

        // Takes two arguments, then determines the difference between them
        // Make 'a' the desired state, 'b' the existing one
        //
        // This is pretty important for angular PID controllers, since
        // transformations from 3 to 2 degrees shouldn't be seen as moving
        // all the way around the unit circle
        //
        // TO_DO: Introduce case loops for when a | b equal 0

        if a >= 0f32 && b >= 0f32 // CASE 1
        {
            if a > b { (a - b) }
            else if a < b { (a - b) }
            else { 0f32 }
        }
        else if a <= 0f32 && b <= 0f32 // CASE 2
        {
            if a > b { (a - b) }
            else if a < b { (a - b) }
            else { 0f32 }
        }
        else if a >= 0f32 && b <= 0f32 // CASE 3
        {
            if a > b { (a - b) }
            else if a < b { panic!("something's very wrong"); }
            else { 0f32 }
        }
        else if a <= 0f32 && b >= 0f32 // CASE 4
        {
            if a > b { panic!("something's very wrong");}
            else if a < b { (a - b) }
            else { 0f32 }
        }
        else {
            println!("hmm, either we're at t=0, or an unexpected condition has appeared (a | b == 0)");
            0f32
        }
}

impl State {

    pub fn update_w_smd(&mut self, desired: State, smd: SMD) {
        //let error_v = find_error(desired.velocity.x,self.velocity.x);
        let error = find_error(desired.position.x,self.position.x);
        println!("desired.position.x = {}, error = {}",desired.position.x,error);

        self.acceleration.x = (-smd.c/smd.m)*self.velocity.x + (smd.k/smd.m)*error;
        self.velocity.x = self.velocity.x + self.acceleration.x*DT;
        self.position.x = self.position.x + self.velocity.x*DT;

    }

}
