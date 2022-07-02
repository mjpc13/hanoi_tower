use std::error::Error;
use std::env;
use std::io::Write;

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
        match stacks {
            Ok(stack) => Ok(Config{stack}),
            Err(error) => Err("Error parsing the input"),
        }
        
    }
}

fn prompt() -> String{
    let mut line = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}

fn play(config: &Config) -> Result<(), Box<dyn Error>> {

    let mut board = engine::Board::new(&config.stack);
    board.update_string();
    //println!("Hanoi tower with {} stacks", config.stack);
    println!("{}", board);

    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let menu = String::from("--- MAIN MENU ---: \n(A) Play the game\n(B) Exit\n> ");
    loop {
        print!("{}", menu);

        match prompt().as_str() {
           "A" => play(&config),
           "B" | "exit" => break Ok(()),
           _ => todo!(),
        };
    }

}
