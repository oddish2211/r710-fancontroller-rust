# r710-fancontroller-rust

This is a rewrite of https://github.com/tomaustin700/r710-FanController-Bash in rust using lm-sensors and ipmitool.
The original used ipmitool to fetch the termperature, but unfortunately my R710 only shows the ambiant temeprature, which does not change as fast as the actual core temperature.

