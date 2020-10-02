# Mandwm
## A dwm and dmenu daemon
This wip daemon will allow an application to be able to interact with features in suckless tools (mainly dwm and dmenu).

## Features
None currently

## Planned Features
* Pipe into dmenu and then execute a process rather than returning a string.
* Set the dwm title bar, append an item, and cycle through alternate title bar names.
* Allow applications to send notifications/warnings through the title bar.
* Quit keybind shows a confirmation menu before exiting.
* Run scripts on init and cleanup
* Provide a Rust API to be able to send messages easily in Rust apps.
* Add features for patches like dualbars.

### Potential Features (Unlikely to be implemented)
* Wrap over dwm so that scripts can be cleaned up more easily
* Compile dwm into mandwm (might be faster?) and add patches as cargo features

## Dependencies
dbus, rustc and cargo.
Ubuntu servers need the package `libdbus-1-dev` to compile and test the program.
