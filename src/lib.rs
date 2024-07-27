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
    use csv::ReaderBuilder;
    use std::{error::Error, fs, io};
    pub enum ProgramStates {
        Menu,
    }
    pub enum Step<T> {
        Continue(T),
        Cancel,
    }
    fn get_user_input() -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        input.trim().to_string()
    }
    fn get_user_num() -> u64 {
        let parsed_input: u64 = get_user_input().parse().unwrap_or_else(|_| {
            eprintln!("The input was not a number. Please try again.");
            get_user_num()
        });
        parsed_input
    }
    pub fn main_menu() {
        println!("Welcome to EcolysisCMD, a Rust tool for ecologicial simulation and analysis.");
        println!("Type the number next to the action you wish to perform and press enter.");
        println!("[1] Deterministic Population Viability Analysis");
        let input = get_user_num() as u32;
    }
    fn get_csv() -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        let binding = get_file();
        let raw_string = binding.as_str();
        let mut rdr = ReaderBuilder::new().from_reader(raw_string.as_bytes());
        let mut result = Vec::new();

        for record in rdr.records() {
            let record = record?;
            result.push(record.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        }

        Ok(result)
    }
    fn get_float_csv_from_str_csv(
        input: Vec<Vec<String>>,
    ) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
        let mut list: Vec<Vec<f64>> = Vec::new();
        let mut temp: Vec<f64> = Vec::new();
        for i in input {
            for j in i {
                temp.push(j.parse()?);
            }
            list.push(temp);
            temp = vec![];
        }
        Ok(list)
    }
    fn get_file() -> String {
        let file_path = get_user_input();
        let contents = fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("File could not be read. Please try again.");
            get_file()
        });
        contents.trim().to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::get_float_csv_from_str_csv;
        #[test]
        fn conversion_from_string_csv_to_float_csv() {
            let test = get_float_csv_from_str_csv(vec![
                vec![String::from("2.1"), String::from("4")],
                vec![String::from("6"), String::from("8.947")],
            ])
            .unwrap();
            assert_eq!(
                test,
                vec![vec![2.1 as f64, 4.0 as f64], vec![6 as f64, 8.947 as f64]]
            );
        }
    }
}
