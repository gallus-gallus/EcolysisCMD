//! This module contains functions to simulate population demographics (not including genetics) using forward-direction population-level simulations. Populations are represented by matrices and vectors containing demographic and behavioral information.

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
    matrix: Vec<LifestageSurvivalVector>,
}
impl PopulationMatrix {
    /// This function builds a Population Matrix from Lifestage Survival Vectors, ensuring that it contains a consistent
    /// number of lifestages across all inputted Lifestage Survival Vectors and in the number of
    /// inputted Lifestage Survival Vectors. If these conditions are not met, it will return an
    /// error message.
    pub fn build(input: Vec<LifestageSurvivalVector>) -> Result<PopulationMatrix, &'static str> {
        if input.len() == input[0].get_vector().len() {
            for count in 1..input.len() {
                if input[count].get_vector().len() != input[count - 1].get_vector().len() {
                    return Err("All lifestage survival vectors must be of matching lengths to construct a population matrix.");
                }
            }
            return Ok(PopulationMatrix { matrix: input });
        } else {
            return Err("Number of items in lifestages must match number of inputted lifestages.");
        }
    }
    pub fn get_matrix(&self) -> &Vec<LifestageSurvivalVector> {
        return &self.matrix;
    }
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
/// Although this is theoretically impossible, the program could also panic if it recieves and out-of-bounds index request for the population vector. However, the function checks for this earlier in order to return a useful error code, so should neever occur.
/// # Esamples
/// ```
/// use ecolysis_cmd::populations::population_level_simulation::{LifestageSurvivalVector,
/// PopulationMatrix, popmatrix_by_popvector, PopulationVector};
///
/// let popvector = PopulationVector::new(vec![150.0, 200.0, 33.0]);
///
/// let mut lifestage_recruit: Vec<LifestageSurvivalVector> = vec![LifestageSurvivalVector::new(vec![0.25, 0.001, 0.75])];
/// lifestage_recruit.push(LifestageSurvivalVector::new(vec![0.3, 0.4346, 0.002]));
/// lifestage_recruit.push(LifestageSurvivalVector::new(vec![0.98, 0.66, 0.161]));
///
/// let popmatrix = PopulationMatrix::build(lifestage_recruit).unwrap();
///
/// let new_popvector = popmatrix_by_popvector(&popmatrix, &popvector);

/// println!("{:?}", new_popvector.unwrap().get_vector());

/// ```
pub fn popmatrix_by_popvector(
    matrix: &PopulationMatrix,
    vector: &PopulationVector,
) -> Result<PopulationVector, &'static str> {
    if matrix.get_matrix().len() != vector.get_vector().len() {
        return Err("the length of inputted population matrix and population vector do not match.");
    }
    let mut new_population_vector: Vec<f64> = vec![];
    for (count, lifestage) in matrix.get_matrix().iter().enumerate() {
        let total: f64 = lifestage.get_vector().iter().sum();
        new_population_vector
            .push(total * vector.get_value_at_index(count as u32)
            .expect("Unexpected Error: the length of inputted population matrix and population vector do not match."));
        // This .expect should never panic. The earlier check for matching vector lengths should ensure this.
    }
    Ok(PopulationVector::new(new_population_vector))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_multiplication() {
        let popvector = PopulationVector::new(vec![150.0, 200.0, 33.0]);
        let mut lifestage_recruit: Vec<LifestageSurvivalVector> =
            vec![LifestageSurvivalVector::new(vec![0.25, 0.001, 10.0])];
        lifestage_recruit.push(LifestageSurvivalVector::new(vec![100.0, 0.4346, 2.0]));
        lifestage_recruit.push(LifestageSurvivalVector::new(vec![66.0, 117.0, 4.0]));
        let popmatrix =
            PopulationMatrix::build(lifestage_recruit).expect("Invalid population matrix."); // Calculated vectors for tests: vec![vec![37.5, 0.15, 1500], vec![20000, 86.92, 400], vec![2178, 3861, 132]];
        let result: Vec<f64> = vec![1537.65, 20486.92, 6171.0];
        assert_eq!(
            &result,
            &popmatrix_by_popvector(&popmatrix, &popvector)
                .unwrap()
                .get_vector()
                .into_iter()
                .map(|x| { (x * 1000000.0).round() / 1000000.0 }) // Rounding is necessary to get rid of floating point errors.
                .collect::<Vec<_>>(),
        );
    }
    #[test]
    fn matrix_invalid_matrix_length() {
        assert!(PopulationMatrix::build(vec![
            LifestageSurvivalVector::new(vec![0.5, 0.7, 0.3]),
            LifestageSurvivalVector::new(vec![0.1, 0.11, 0.6])
        ])
        .is_err());
    }
    #[test]
    fn matrix_unmatched_lifestage_lengths() {
        assert!(PopulationMatrix::build(vec![
            LifestageSurvivalVector::new(vec![0.5, 0.7, 0.3]),
            LifestageSurvivalVector::new(vec![0.1, 0.11, 0.6]),
            LifestageSurvivalVector::new(vec![0.2, 0.91])
        ])
        .is_err());
    }
}
