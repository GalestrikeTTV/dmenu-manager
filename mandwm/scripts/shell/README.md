# Shell scripts
These scripts will be executed at every stage of updating the dwm bar. Mandwm will read stdout and strip away any newlines before adding it to the statusbar.
Shell scripts should be updated asyncronously, so blocking calls shouldn't matter to the performance of mandwm. 

## Unaddressed concerns
* Scripts cannot define how often they can run.
* Scripts cannot define their own priority.
* Error handling is bound to stderr or status codes. How do we determine how something fails? (status codes are nonstandard except for 0)
* Spawning a shell is slow.

### NOTE: This is all subject to change
Shell scripts might run on a loop instead so they can define their own update time. I'll list my concerns with a few of the approaches I might take.

#### Run each shell script in a job queue (tokio?)
* Scripts cannot define how often they should run (unless they can export a variable that says how often they need to run)
* Scripts constantly have to spawn new shells

#### Run each shell script in a for loop (and have them sleep)
* Mandwm cant send events to the scripts easily except by constantly checking the DBUS.
* Running solely by shell instead of executable might be slower. (Maybe not as slow as constantly spawning shells)
* We can only get return values by DBUS.
