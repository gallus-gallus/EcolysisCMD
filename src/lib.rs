pub mod populations;

pub fn test() {
    println!("This is the only thing now!");
}

mod interface {
    pub enum ProgramStates {
        Menu,
    }
}
