# RustSample
Rust sample : soccer championship management

This program does not fill all the functionalities it should. It is only a basic soccer championship management program.

Perform the following steps to use this program:
- Install cargo : https://crates.io/ .
- Compile ```rustc src/main.rs``` .
- Execute ```./src/main``` .

This championship management program works running command from shell.
With cargo, you have to write cargo run -- [arg]

with [arg] among the below arguments:

    calendar    shows championship calendar
    create      creates championship from teams enumerated in TEAMS_FILE file
    help        Prints this message or the help of the given subcommand(s)
    rankings    show rankings of the championship
    teams       shows teams list
    update      updates one result of the championship
    
when using update subcommand, you need to specify 3 arguments corresponding respectively to the id of the match you want to update, the id of the receiving team, and the id of the coming team.
