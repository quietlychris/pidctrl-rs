## pidctrl-rs

This is a (currently small) control systems library written in Rust. It
is primarily for educational use, and is tied to a small Python script
using bash to quickly generate all results.

The current implementation includes basic visualizations for both PID control
logic and spring-mass-damper systems.

To run, please do the following:
  1. Make sure all the dependencies exist, including Python's *numpy*, *scipy*, as well as Rust's *cargo*
  2. Create a new Rust project
  3. Copy both `main.rs` and `pid_lib.rs` into the `/project_name/src/`
  4. Change the gains and time period in `main.rs` to those desired
  5. Copy `run.sh` and `graph.py` into `/project_name/`
  6. Make sure the script has the right permissions, and run using `$ ./run.sh`

A graph displaying the system's behavior should be generated.

Please feel free to voice questions, comments, or concerns, as well as
to make any pull requests. Thanks!
