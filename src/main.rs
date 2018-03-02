mod pid_lib;

use pid_lib::*;
use std::fs::File;
use std::io::{Write, BufWriter};

fn main(){

    // Creates a .csv file to help visualize resulting behavior
    let filename = "run_log.csv";
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);


    // Defines a control value's position, and the desired setpoint
    //let mut position = Position {x: 0.0 };
    //let mut setpoint = Position {x: 1.0 };
    let mut state = State { position: Position {x: 0.0} ,
                            velocity: Velocity {x: 0.0} ,
                            acceleration: Acceleration {x: 0.0}
                          };

    let desired =  State { position: Position {x: 1.0},
                           velocity: Velocity {x: 0.0},
                           acceleration: Acceleration {x: 0.0}
                         };

    // Creates a new controller
    //let mut controller = PIDController::new(1.5,1.0,0.0,PIDSet::default());
    let smd = SMD::new(1.0,0.3,1.0);

    // Sets 'time' to zero and runs loop loop for designatd time
    let mut time: f32 = 0.0;
    while time < 50.0
    {
        state.update_w_smd(desired,smd);
        // The following two lines are for command-line output
        //println!("  time = {:.2} s   | {:.2}  {:.2}  {:.2}     "
        //    ,time,state.position.x,state.velocity.x,state.acceleration.x);
        // Uses time-step value pulled from library
        time = time + pid_lib::DT;

        // writing data the the csv file
        f.write_all(time.to_string().as_bytes())
            .expect("Couldn't write time to file");
        f.write_all(b",")
            .expect("Comma was not written to file");
        f.write_all(state.position.x.to_string().as_bytes())
            .expect("Couldn't write position to file");
        f.write_all(b"\n")
            .expect("Couldn't append endline to file");

    }
    // For plotting PID controller
    /*
    time = 0.0;
    while time < 50.0
    {
        position = controller.control(position,setpoint);
        // The following two lines are for command-line output
        //println!("  time = {:.2} s   | {:.5}   "
        //    ,time,position.x);
        // Uses time-step value pulled from library
        time = time + pid_lib::DT;

        // writing data the the csv file
        f.write_all(time.to_string().as_bytes())
            .expect("Couldn't write time to file");
        f.write_all(b",")
            .expect("Comma was not written to file");
        f.write_all(position.x.to_string().as_bytes())
            .expect("Couldn't write position to file");
        f.write_all(b"\n")
            .expect("Couldn't append endline to file");

    }
    */

}
