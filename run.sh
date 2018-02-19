#!/bin/bash
echo '***Beginning script run process, deleting previous log'
# Please note that the filename is decided in 'main.rs'
rm -f run_log.csv
clear
echo '***Running cargo'
cargo run
echo '***Plotting graph'
python graph.py
