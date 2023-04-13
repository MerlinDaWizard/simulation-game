# First draft of simulation state management

Add a second set of bevy `State`s, this would be `SimState`:
    - Halted
    - Paused
    - Active

I would prefer to do states (storing data) inside states with this however bevy does not support these in v0.10

Work around is to store a Resource called `RunType`. This resource defines how the simulation is run
E.g. 
    - None
    - Continuous
    - Step(usize), value representing the amount of steps remaining until it moves into the `PAUSED` `SimState`


When in Pause state we should ignore an value of RunType and then CHANGE IT WHEN EXITING STATE

OnExit --> Active, runType = None
OnEnter --> Active, set runType to corrisponding button action
OnEnter --> Halted, reset sim


On exit InGame, move SimState to halted