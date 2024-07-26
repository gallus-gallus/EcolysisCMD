pub mod populations;
use interface::main_menu;
pub use populations::population_level_simulation::{
    PopulationMatrix, PopulationVector, PvaDeterministicOutput, PvaDeterministicPopulation,
};

pub fn run() {
    println!("Welcome to EcolysisCMD, a Rust tool for ecologicial simulation and analysis.");
    main_menu();
}

mod interface {
    use std::io;
    pub enum ProgramStates {
        Menu,
    }
    fn get_user_menu_num() -> u32 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        let parsed_input: u32 = input.trim().parse().unwrap_or_else(|_| {
            println!("The input was not a number. Please try again.");
            get_user_menu_num()
        });
        parsed_input
    }
    pub fn main_menu() {
        println!("Welcome to EcolysisCMD, a Rust tool for ecologicial simulation and analysis.");
        println!("Type the number next to the action you wish to perform and press enter.");
        println!("[1] Deterministic Population Viability Analysis");
        let input = get_user_menu_num();
    }
}
