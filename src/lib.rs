use analysis::PopulationVector;

pub fn test() {
    println!("This is the only thing now!");
    let popvec = analysis::PopulationVector::new(vec![5, 3, 2]);
    let popmat = analysis::PopulationMatrix {
        matrix: vec![vec![10, 9, 8], vec![55, 100, 200], vec![7, 3, 2]],
    };
    print!("{:?}", analysis::matrix_by_vector(&popmat, &popvec).vector)
}

mod interface {
    pub enum ProgramStates {
        Menu,
    }
}

mod analysis {
    use std::process::Output;

    pub struct PopulationVector {
        pub vector: Vec<u32>,
    }
    impl PopulationVector {
        pub fn new(vector: Vec<u32>) -> PopulationVector {
            PopulationVector { vector: vector }
        }
    }
    pub struct PopulationMatrix {
        pub matrix: Vec<Vec<u32>>,
    }
    pub fn matrix_by_vector(
        matrix: &PopulationMatrix,
        vector: &PopulationVector,
    ) -> PopulationVector {
        let mut vector_iter = vector.vector.iter();
        PopulationVector::new(
            matrix
                .matrix
                .iter()
                .map(|value| {
                    let current_vector_value = vector_iter.next();
                    value
                        .iter()
                        .map(|value2| {
                            value2
                                * current_vector_value.unwrap_or_else(|| {
                                    let hi: &u32 = &32;
                                    return hi;
                                })
                        })
                        .sum()
                })
                .collect(),
        )
    }
}
