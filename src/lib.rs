use std::error::Error;
use std::env;

mod engine;

pub struct Config{
    pub stack: i32
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        let arg = match args.len() {
            1 => String::from("3"),
            2 => args[1].clone(),
            _ => panic!("Too many arguments to parse")
        };

        let stacks = arg.parse();
        let stacks = match stacks {
            Ok(stack) => Ok(Config{stack}),
            Err(error) => Err("Error parsing the input"),
        };

        stacks
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    

    let mut board = engine::Board::new(&config.stack);
    println!("Hanoi tower with {} stacks", config.stack);
    println!("{}", board);

    Ok(())
}
