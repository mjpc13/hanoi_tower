use std::env;
use std::process;

use hanoi_tower::Config;

/*  Process to keep your code organized:
 *      - Split your program into a main.rs and a lib.rs and move your program's logic to lib.rs
 *      - As long as your command line parsing logic is small, it can remain in main.rs
 *      - When the command line parsing logic starts getting complicated, exctract it from main.rs and
 *        move it to lib.rs
 *
 *
 *  The main.rs responsabilities should be:
 *      - Calling the command parsing logic with the arguments values
 *      - Setting up any other configuration
 *      - Calling a run function in lib.rs
 *      - Handling the error if run returns an error
 */

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1);
    });

    if let Err(_e) = hanoi_tower::run(config) {
        process::exit(1);
    };
    
}
