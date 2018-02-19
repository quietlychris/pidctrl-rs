###pidctrl-rs

This is a (currently small) PID controller library written in Rust. It
is primarily for educational use, and is tied to a small Python script
using bash to quickly generate all results.

The workflow would follow as such:
  1. Dependencies include Python's *numpy*, *scipy*, and Rust's *cargo*
  2. Create a new Rust project
  3. Copy both 'main.rs' and 'pid_lib.rs' into the '/project_name/src/'
  4. Change the gains and time period in 'main.rs' to those desired
  5. Copy 'run.sh' and 'graph.py' into '/project_name/'
  6. Run using '$ ./run.sh'

A graph displaying the system's behavior should be generated
