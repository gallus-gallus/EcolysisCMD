//! This module contains functions to simulate population demographics (not including genetics) using forward-direction population-level simulations. Populations are represented by matrices and vectors containing demographic and behavioral information.
#[derive(Clone)]
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
    pub fn get_lifestage_count(&self) -> u8 {
        return self.lifestage_count;
    }
}

pub struct PopulationMatrix {
    matrix: Vec<Vec<f64>>,
    lifestage_count: u8,
}
impl PopulationMatrix {
    /// This function builds a Population Matrix from a square vector of vectors, ensuring that it contains a consistent
    /// number of lifestages across all inputted Lifestage Survival Vectors and in the number of
    /// inputted Lifestage Survival Vectors. If these conditions are not met, it will return an
    /// error message.
    pub fn build(input: Vec<Vec<f64>>) -> Result<PopulationMatrix, &'static str> {
        if input.len() == input[0].len() {
            for count in 1..input.len() {
                if input[count].len() != input[count - 1].len() {
                    return Err("All sub-vectors must be of matching lengths to construct a population matrix.");
                }
            }
            return Ok(PopulationMatrix {
                lifestage_count: input.len() as u8,
                matrix: input,
            });
        } else {
            return Err("Number of items in lifestages must match number of inputted sub-vectors.");
        }
    }
    pub fn get_lifestage_count(&self) -> u8 {
        return self.lifestage_count;
    }
    pub fn get_matrix(&self) -> &Vec<Vec<f64>> {
        return &self.matrix;
    }

    /// # Multiply a Population Matrix by a Population vector
    /// ## Contract
    /// Given an input of a PopulationMatrix and a PopulationVector with the same number of items
    /// in their `matrix` and `vector` values respectively, the function will return a
    /// PopulationMatrix.
    /// ##Use
    /// This function multiplies a vector by a matrix, but does so in the context of relevant types.
    /// This function takes a population matrix (formatted as a PopulationMatrix struct) and a population vector (formatted as a PopulationVector struct) as an imput, calculating the sum of the components of each LifeStageSurvivalVector within the population matrix and muliplying it by the value of the population vector at the same index. It then returns a new PopulationVector.
    ///
    /// This can be visually thought of as such:
    ///
    ///[1]   [11,  2, 0.5 ]   [ 13.5]
    ///[5] x [ 1, 10, 3   ] = [ 70  ]
    ///[8]   [20,  6, 0.25]   [210  ]
    ///
    ///This calculation is common in Population Variability Analysis (PVA) wherein each row (LifeStageSurvivalVector) represents the probability of recruitment into that life stage over the course of a year. By multiplying a matrix of these probabilities by a vector containing the current population, a researcher can estimate the following year's population.
    ///
    /// ## Errors
    /// This function will return an Err('static str') if the number of life stage items in the matrix is not equal to the number of items in the population vector.
    ///
    /// Although this is theoretically impossible, the program could also panic if it recieves and out-of-bounds index request for the population vector. However, the function checks for this earlier in order to return a useful error code, so should never occur.
    /// # Examples
    /// ```
    /// use ecolysis_cmd::populations::population_level_simulation::{PopulationMatrix, PopulationVector};
    ///
    /// let popvector = PopulationVector::new(vec![150.0, 200.0, 33.0]);
    ///
    /// let mut lifestage_recruit: Vec<Vec<f64>> = vec![vec![0.25, 0.001, 0.75]];
    /// lifestage_recruit.push(vec![0.3, 0.4346, 0.002]);
    /// lifestage_recruit.push(vec![0.98, 0.66, 0.161]);
    ///
    /// let popmatrix = PopulationMatrix::build(lifestage_recruit).unwrap();
    ///
    /// let new_popvector = popmatrix.project_vector(&popvector);

    /// println!("{:?}", new_popvector.unwrap().get_vector());

    /// ```
    pub fn project_vector(
        &self,
        vector: &PopulationVector,
    ) -> Result<PopulationVector, &'static str> {
        if self.lifestage_count != vector.get_lifestage_count() {
            return Err(
                "the length of inputted population matrix and population vector do not match.",
            );
        }
        let mut new_population_vector: Vec<f64> = vec![];
        for (count, lifestage) in self.matrix.iter().enumerate() {
            let total: f64 = lifestage.iter().sum();
            new_population_vector
                .push(total * vector.get_value_at_index(count as u32)
                .expect("Unexpected Error: the length of inputted population matrix and population vector do not match."));
            // This .expect should never panic. The earlier check for matching vector lengths should ensure this.
        }
        Ok(PopulationVector::new(new_population_vector))
    }
}

pub struct PvaPopulation {
    initial_population: PopulationVector,
    projection_matrices: Vec<PopulationMatrix>,
}
impl PvaPopulation {
    pub fn build(
        init_pop: PopulationVector,
        matrices: Vec<PopulationMatrix>,
    ) -> Result<PvaPopulation, &'static str> {
        let expected_lifestage_length = init_pop.get_lifestage_count();
        if matrices[0].get_lifestage_count() != expected_lifestage_length {
            return Err("Population vector size does not match matrices.");
        }
        if matrices.len() > 1 {
            for i in 1..matrices.len() {
                if matrices[i].get_lifestage_count() != expected_lifestage_length {
                    return Err("Matrices are not of consistent length.");
                }
            }
        }
        Ok(PvaPopulation {
            initial_population: init_pop,
            projection_matrices: matrices,
        })
    }
    pub fn deterministic_projection(&self, iterations: u32) -> Result<PvaOutput, &'static str> {
        if self.projection_matrices.len() > 1 {
            return Err("Deterministic Population Viability Analysis is only avaialble for PvaPopulation instances containing a single prjection matrix.");
        } else {
            let mut active_vector = self.initial_population.clone();
            let mut result: Vec<PopulationVector> = Vec::new();
            for _ in 1..=iterations {
                active_vector = self.projection_matrices[0].project_vector(&active_vector)?;
                result.push(active_vector.clone());
            }
            return Ok(PvaOutput::Deterministic(result));
        }
    }
}

pub enum PvaOutput {
    Deterministic(Vec<PopulationVector>),
    Stochastic,
}
impl PvaOutput {
    pub fn print_output(&self) {
        match self {
            PvaOutput::Deterministic(data) => {
                let mut string = String::new();
                for (counti, i) in data.iter().enumerate() {
                    for (countj, j) in i.get_vector().iter().enumerate() {
                        string.push_str(&j.to_string());
                        if countj + 1 < i.get_vector().len() {
                            string.push_str(", ");
                        }
                    }
                    if counti + 1 < data.len() {
                        string.push_str("\n");
                    }
                }
                println!("{}", string);
            }
            PvaOutput::Stochastic => {
                eprintln!("Not implemented.")
            }
        }
    }
    pub fn return_determinsitic_output(&self) -> Result<&Vec<PopulationVector>, &'static str> {
        match self {
            PvaOutput::Deterministic(data) => Ok(data),
            PvaOutput::Stochastic => {
                Err("This function is only avaialble for determinisitc models.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_multiplication() {
        let popvector = PopulationVector::new(vec![150.0, 200.0, 33.0]);
        let mut lifestage_recruit: Vec<Vec<f64>> = vec![vec![0.25, 0.001, 10.0]];
        lifestage_recruit.push(vec![100.0, 0.4346, 2.0]);
        lifestage_recruit.push(vec![66.0, 117.0, 4.0]);
        let popmatrix =
            PopulationMatrix::build(lifestage_recruit).expect("Invalid population matrix."); // Calculated vectors for tests: vec![vec![37.5, 0.15, 1500], vec![20000, 86.92, 400], vec![2178, 3861, 132]];
        let result: Vec<f64> = vec![1537.65, 20486.92, 6171.0];
        assert_eq!(
            &result,
            &popmatrix
                .project_vector(&popvector)
                .unwrap()
                .get_vector()
                .into_iter()
                .map(|x| { (x * 1000000.0).round() / 1000000.0 }) // Rounding is necessary to get rid of floating point errors.
                .collect::<Vec<_>>(),
        );
    }
    #[test]
    fn matrix_invalid_matrix_length() {
        assert!(PopulationMatrix::build(vec![vec![0.5, 0.7, 0.3], vec![0.1, 0.11, 0.6]]).is_err());
    }
    #[test]
    fn matrix_unmatched_lifestage_lengths() {
        assert!(PopulationMatrix::build(vec![
            vec![0.5, 0.7, 0.3],
            vec![0.1, 0.11, 0.6],
            vec![0.2, 0.91]
        ])
        .is_err());
    }
    #[test]
    fn pva_simple_matrix_projection_test() {
        let population_vec = PopulationVector::new(vec![15.0, 155.0, 200.0]);
        let matrix = PopulationMatrix::build(vec![
            vec![0.0, 0.0, 0.7],
            vec![0.5, 0.8, 0.0],
            vec![0.0, 0.7, 0.91],
        ])
        .unwrap();
        let population = PvaPopulation::build(population_vec, vec![matrix]).unwrap();
        let result = population.deterministic_projection(8).unwrap();
        result.print_output();
        let correct_result = vec![0.9, 1264.4, 9028.9];
        let mut temp_vec: Vec<f64> = Vec::new();
        let mut clean_output: Vec<Vec<f64>> = Vec::new();
        for i in result.return_determinsitic_output().unwrap() {
            for j in i.get_vector() {
                temp_vec.push(((j * 10.0 as f64).round()) / 10.0);
            }
            clean_output.push(temp_vec);
            temp_vec = vec![];
        }
        assert_eq!(correct_result, clean_output[clean_output.len() - 1])
    }
}
