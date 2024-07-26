//! This module contains functions to simulate population demographics (not including genetics) using forward-direction population-level simulations. Populations are represented by matrices and vectors containing demographic and behavioral information.

/// This struct represents a population by a (typically integer) vector. Each value of the vector represents the number of individuals in a lifestage present in the population. For example, a population with 15 hatchlings, 8 juveniles, and 30 adults could be represented by this vector: `[40, 20, 100]`. This struct is meant to contain this type of information. The data is stored as f64 (floating point) values to accommodate conditions when decimal populations are desirable and facilitate calculations that may not return integer values.
#[derive(Clone)]
pub struct PopulationVector {
    vector: Vec<f64>,
    lifestage_count: u8,
}
impl PopulationVector {
    /// Create a new Population Vector instance by inputting a vector containing f64 values.
    pub fn new(vector: Vec<f64>) -> PopulationVector {
        PopulationVector {
            lifestage_count: vector.len() as u8,
            vector: vector,
        }
    }
    /// Return the value stored at a specifc index in the Population Vector based on inputed
    /// integer (u32). The first value is 0.
    pub fn get_value_at_index(&self, index: u32) -> Option<&f64> {
        self.vector.get(index as usize)
    }
    // Return full vector stored in the Population Vector as a `Vec<f64>`.
    pub fn get_vector(&self) -> &Vec<f64> {
        return &self.vector;
    }
    // Return the number of items stored in the Population Vector instance. This is used to prevent
    // errors in calculations that require matching vector/matrix lengths.
    pub fn get_lifestage_count(&self) -> u8 {
        return self.lifestage_count;
    }
}

/// This struct represents the likelihood of different lifestages of an organism to survive, grow,
/// and reproduce to the next lifestage (or stay in the same lifestage) by storing decimal (f64) values in a matrix (a vector of vectors, denoted
/// `Vec<Vec<f64>>`). Each sub-vector (row) represents a lifestage, and each item (column)
/// represents the yearly proportion of individuals recruited to that lifestage. The first row typically represents newborns, seedlings, etc.
///
/// ## Examples
/// Index [1][1] is the number of lifestage 1 individuals that were in lifestage 1 the
/// previous year.
///
/// Index [1][2] is the number of of lifestage 1 indidviduals that were in lifestage 2 the year
/// before.
///
/// Indeg [2][1] is the number of lifestage 2 individuals who were lifetage 1 individuals the
/// previous year.
///
/// ...etc...
///
/// An example matrix could look like this:
/// [0][0][0.1][0.2]
/// [0.6][0][0][0]
/// [0][0.8][0][0]
/// [0][0][0.8][0.94]
pub struct PopulationMatrix {
    matrix: Vec<Vec<f64>>,
    lifestage_count: u8,
}
impl PopulationMatrix {
    /// This function builds a Population Matrix from a square vector of vectors (Vec<Vec<f64>>), ensuring that it contains a consistent
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
    /// Returns the number of listages represented in the Population Matrix, useful for calculations requiring
    /// matching numbers of lifestages.
    pub fn get_lifestage_count(&self) -> u8 {
        return self.lifestage_count;
    }
    /// Returns the full matrix of matrices stored in the Population Matrix.
    pub fn get_matrix(&self) -> &Vec<Vec<f64>> {
        return &self.matrix;
    }

    /// Given an input of a PopulationMatrix and a PopulationVector with the same number of items
    /// in their `matrix` and `vector` values respectively, the function will return a
    /// PopulationMatrix.
    /// ##Use
    /// This function takes a population matrix (formatted as a PopulationMatrix struct) and a population vector (formatted as a PopulationVector struct) as an imput, calculating the sum of the components of each column of the vector within the population matrix and muliplying it by the value of the population vector at the same index. This is essentially a column-wise matrix-vector product. It then returns a new PopulationVector.
    ///
    /// This can be visually thought of as such:
    ///
    ///[ 40]   [0  , 0  , 0.1 ]   [ 10]
    ///[ 20] x [0.6, 0.8, 0   ] = [ 40]
    ///[100]   [0  , 0.8, 0.95]   [111]
    ///
    ///This calculation is common in Population Variability Analysis (PVA) wherein each row (LifeStageSurvivalVector) represents the probability of recruitment into that life stage over the course of a year. By multiplying a matrix of these probabilities by a vector containing the current population, a researcher can estimate the following year's population.
    ///
    /// ## Errors
    /// This function will return an Err('static str') if the number of rows or items within rows in the matrix is not equal to the number of items in the population vector.
    ///
    /// Although this is theoretically impossible, the program could also panic if it recieves an out-of-bounds index request for the population vector. However, the function checks for this earlier in order to return a useful error code and prevent other mistakes, so should never occur.
    /// # Examples
    /// ```
    /// use ecolysis_cmd::populations::population_level_simulation::{PopulationMatrix, PopulationVector}; // import relevant structs
    /// let popvector = PopulationVector::new(vec![150.0, 200.0, 33.0]); // create a population vector type
    /// let popmatrix = PopulationMatrix::build(vec![
    /// vec![0.25, 0.001, 0.75],
    /// vec![0.3, 0.4346, 0.002],
    /// vec![0.98, 0.66, 0.161]
    /// ]).unwrap(); // create a Population Matrix type
    /// let new_popvector = popmatrix.project_vector(&popvector); // project the vector by the matrix
    /// println!("{:?}", new_popvector.unwrap().get_vector()); // print results
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
        let mut storage: f64 = 0.0;
        for lifestage in &self.matrix {
            for (count2, item) in lifestage.iter().enumerate() {
                storage += item
                    * vector
                        .get_value_at_index(count2 as u32)
                        .expect("Index out of bounds.");
            }
            new_population_vector.push(storage);
            storage = 0.0;
        }
        Ok(PopulationVector::new(new_population_vector))
    }
}

/// The PvaDeterministicPopulation struct stores population data for deterministic PVA models, allowing PVA operations to be performed by simply calling
/// functions on an instance. It contains the following:
/// - A Population Vector representing the initial population size.
/// - A Population Matrix contains data on the survival rates
/// and recruitment rates of verious lifestages.
pub struct PvaDeterministicPopulation {
    initial_population: PopulationVector,
    projection_matrix: PopulationMatrix,
}
impl PvaDeterministicPopulation {
    /// Return a Result enum containing a new PvaDeterministicPopulation instance with the input of a Population Vector and a Population Matrix.    /// one Population Matrix in the latter vector.
    /// # Errors
    /// Will return `Err<'static str>` if the lengths of the Population Vector the Matrix do not match.
    pub fn build(
        init_pop: PopulationVector,
        matrix: PopulationMatrix,
    ) -> Result<PvaDeterministicPopulation, &'static str> {
        let expected_lifestage_length = init_pop.get_lifestage_count();
        if matrix.get_lifestage_count() != expected_lifestage_length {
            return Err("Population vector size does not match matrices.");
        }
        Ok(PvaDeterministicPopulation {
            initial_population: init_pop,
            projection_matrix: matrix,
        })
    }
    // Return a Result enum containing a PvaDeterministicOutput type that holds the output of a determinisitc simulation
    // given the number of simulation steps to perform (as a u32). This function performs the
    // actual simulation.
    //
    // The function may panic if the Populatiom Matrix is not square or the
    // lengths of Population Vector and Population Matrix do not match, although this situation should
    // be prevented by checks when building a PVA Population instance.
    pub fn deterministic_projection(&self, iterations: u32) -> PvaDeterministicOutput {
        let mut active_vector = self.initial_population.clone();
        let mut result: Vec<PopulationVector> = Vec::new();
        for _ in 1..=iterations {
            active_vector = self.projection_matrix.project_vector(&active_vector).expect("This error should not be possible. Mismatched Vector and Matrix lengths, or non-square Matrix. Please file a bug report.");
            result.push(active_vector.clone());
        }
        return PvaDeterministicOutput::new(result);
    }
}

/// This enum stores the output of Population Viability Analysis operations performed by the PVA
/// Deterministic Population struct.
pub struct PvaDeterministicOutput {
    result: Vec<PopulationVector>,
}
impl PvaDeterministicOutput {
    // Create a new PvaDeterministicOutput struct from a vector of PopulationVectors
    // (Vec<PopulationVector>).
    pub fn new(simulation_output: Vec<PopulationVector>) -> PvaDeterministicOutput {
        return PvaDeterministicOutput {
            result: simulation_output,
        };
    }
    /// Print a CSV containing the output of each simulation step to the console.
    pub fn print_output(&self) {
        let mut string = String::new();
        for (counti, i) in self.result.iter().enumerate() {
            for (countj, j) in i.get_vector().iter().enumerate() {
                string.push_str(&j.to_string());
                if countj + 1 < i.get_vector().len() {
                    string.push_str(", ");
                }
            }
            if counti + 1 < self.result.len() {
                string.push_str("\n");
            }
        }
        println!("{}", string);
    }
    /// Return a Result enum containg a vector of Population Vectors representing all the data from
    /// each step of the simulation for a determinisitc model.
    pub fn return_output(&self) -> &Vec<PopulationVector> {
        &self.result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_multiplication() {
        let popvector = PopulationVector::new(vec![40.0, 20.0, 100.0]);
        let mut lifestage_recruit: Vec<Vec<f64>> = vec![vec![0.0, 0.0, 0.1]];
        lifestage_recruit.push(vec![0.6, 0.8, 0.0]);
        lifestage_recruit.push(vec![0.0, 0.8, 0.95]);
        let popmatrix =
            PopulationMatrix::build(lifestage_recruit).expect("Invalid population matrix."); // Calculated vectors for tests: vec![vec![37.5, 0.15, 1500], vec![20000, 86.92, 400], vec![2178, 3861, 132]];
        let result: Vec<f64> = vec![10.0, 40.0, 111.0];
        assert_eq!(
            &result,
            &popmatrix
                .project_vector(&popvector)
                .unwrap()
                .get_vector()
                .into_iter()
                .map(|x| { (x * 10.0).round() / 10.0 }) // Rounding is necessary to get rid of floating point errors.
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
        let population_vec = PopulationVector::new(vec![40.0, 20.0, 100.0]);
        let matrix = PopulationMatrix::build(vec![
            vec![0.0, 0.0, 0.1],
            vec![0.6, 0.8, 0.0],
            vec![0.0, 0.8, 0.95],
        ])
        .unwrap();
        let population = PvaDeterministicPopulation::build(population_vec, matrix).unwrap();
        let result = population.deterministic_projection(8);
        result.print_output();
        let correct_result = vec![24.9, 50.8, 273.5];
        let mut temp_vec: Vec<f64> = Vec::new();
        let mut clean_output: Vec<Vec<f64>> = Vec::new();
        for i in result.return_output() {
            for j in i.get_vector() {
                temp_vec.push(((j * 10.0 as f64).round()) / 10.0);
            }
            clean_output.push(temp_vec);
            temp_vec = vec![];
        }
        assert_eq!(correct_result, clean_output[clean_output.len() - 1])
    }
}
