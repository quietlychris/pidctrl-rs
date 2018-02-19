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
    let mut position = Position {x: 0.0 };
    let mut setpoint = Position {x: 1.0 };

    // Creates a new controller
    let mut controller = PIDController::new(1.5,1.0,0.0,PIDSet::new());


    // Sets 'time' to zero and runs control loop for designatd time
    let mut time: f32 = 0.0;
    while time < 1.0
    {
        position = controller.control(position,setpoint);
        println!("  time = {:.2} s   | {:.5}      ",time,position.x);
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

}
