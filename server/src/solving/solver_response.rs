use serde::Deserialize;

use super::solution::Solution;

#[derive(Debug, Deserialize)]
pub struct SolverResponse {
    pub solution: Solution
}