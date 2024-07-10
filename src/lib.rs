use analysis::PopulationVector;

pub fn test() {
    println!("This is the only thing now!");
}

mod interface {
    pub enum ProgramStates {
        Menu,
    }
}

mod analysis {
    use std::process::Output;
    // use std::iter::Enumerate;

    pub struct PopulationVector {
        vector: Vec<f64>,
        lifestage_count: u8,
    }
    impl PopulationVector {
        pub fn new(vector: Vec<f64>) -> PopulationVector {
            PopulationVector {
                lifestage_count: vector.len() as u8,
                vector: vector,
            }
        }
        pub fn get_value_at_index(&self, index: u32) -> Option<&f64> {
            self.vector.get(index as usize)
        }
        pub fn get_vector(&self) -> &Vec<f64> {
            return &self.vector;
        }
    }

    pub struct LifestageSurvivalVector {
        vector: Vec<f64>,
        lifestage_count: u8,
    }
    impl LifestageSurvivalVector {
        pub fn new(vector: Vec<f64>) -> LifestageSurvivalVector {
            LifestageSurvivalVector {
                lifestage_count: vector.len() as u8,
                vector: vector,
            }
        }
        pub fn get_vector(&self) -> &Vec<f64> {
            return &self.vector;
        }
    }
    pub struct PopulationMatrix {
        pub matrix: Vec<LifestageSurvivalVector>,
    }

    pub fn popmatrix_by_popvector(
        matrix: &PopulationMatrix,
        vector: &PopulationVector,
    ) -> Result<PopulationVector, &'static str> {
        if matrix.matrix.len() != vector.vector.len() {
            return Err(
                "the length of inputted population matrix and population vector do not match.",
            );
        }
        let mut new_population_vector: Vec<f64> = vec![];
        for (count, lifestage) in matrix.matrix.iter().enumerate() {
            let total: f64 = lifestage.get_vector().iter().sum();
            new_population_vector
                .push(total * vector.get_value_at_index(count as u32).expect("Unexpected Error: the length of inputted population matrix and population vector do not match."));
        }
        Ok(PopulationVector::new(new_population_vector))
    }
}

#[cfg(test)]
mod tests {
    use analysis::{popmatrix_by_popvector, LifestageSurvivalVector, PopulationMatrix};

    use super::*;

    #[test]
    fn matrix_multiplication() {
        let popvector = PopulationVector::new(vec![150.0, 200.0, 33.0]);
        let mut lifestage_recruit: Vec<LifestageSurvivalVector> =
            vec![LifestageSurvivalVector::new(vec![0.25, 0.001, 10.0])];
        lifestage_recruit.push(LifestageSurvivalVector::new(vec![100.0, 0.4346, 2.0]));
        lifestage_recruit.push(LifestageSurvivalVector::new(vec![66.0, 117.0, 4.0]));
        let popmatrix = PopulationMatrix {
            matrix: lifestage_recruit,
        };
        // let result: Vec<f64> = vec![vec![37.5, 0.15, 1500], vec![20000, 86.92, 400], vec![2178, 3861, 132]];
        let result: Vec<f64> = vec![1537.65, 20486.92, 6171.0];
        assert_eq!(
            &result,
            &popmatrix_by_popvector(&popmatrix, &popvector)
                .unwrap()
                .get_vector()
                .into_iter()
                .map(|x| { (x * 1000000.0).round() / 1000000.0 }) // Rounding is necessary to get
                // rid of floating point errors.
                .collect::<Vec<_>>(),
        );
    }
}
