# Rust scripts
These scripts will be compiled into mandwm and run on every statusbar update. Rust scripts have the potential to be much faster than Shell scripts since they don't have to spawn shells at all.
There will be some helper macros in mandwm-api to help the main crate compile each script into the desired format as well as a configurable build script.

## NOTE: Rust scripts will not be available in mandwm 1.0.
Shell scripts are much easier to use and generally what people use for dwm in general. My goal is to have mandwm's functionality finished before adding additional features.
Rust scripting will be implemented in mandwm 1.1 and is high priority nonetheless.
