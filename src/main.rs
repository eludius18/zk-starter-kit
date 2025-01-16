use crate::field::FieldElement;
use num_bigint::BigInt;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use num_traits::Zero;
use crate::proof::Proof;
use crate::qap::QAP;

/// Represents a variable in the R1CS.
#[derive(Clone, Serialize, Deserialize)]
pub struct Variable {
    /// The index of the variable.
    pub index: usize,
    /// The value of the variable.
    pub value: FieldElement,
}

/// Represents an operation in the R1CS.
#[derive(Serialize, Deserialize)]
pub enum Operation {
    Add,
    Mul,
    Hash,
}

/// Represents a constraint in the R1CS.
#[derive(Serialize, Deserialize)]
pub struct Constraint {
    /// The left side of the constraint.
    pub left: Vec<(Variable, BigInt)>,
    /// The right side of the constraint.
    pub right: Vec<(Variable, BigInt)>,
    /// The output side of the constraint.
    pub output: Vec<(Variable, BigInt)>,
    /// The operation of the constraint.
    pub operation: Operation,
}

/// Represents a Rank-1 Constraint System (R1CS).
#[derive(Serialize, Deserialize)]
pub struct R1CS {
    /// The variables in the R1CS.
    pub variables: Vec<Variable>,
    /// The constraints in the R1CS.
    pub constraints: Vec<Constraint>,
    /// The QAP representation of the R1CS.
    pub qap: QAP,
}

impl R1CS {
    /// Creates a new R1CS instance.
    ///
    /// # Returns
    /// - `Self`: A new instance of the `R1CS` struct.
    pub fn new() -> Self {
        R1CS {
            variables: Vec::new(),
            constraints: Vec::new(),
            qap: QAP::new(),
        }
    }

    /// Adds a constraint to the R1CS and also updates the QAP representation.
    ///
    /// # Parameters
    /// - `left_coeffs`: The coefficients for the left polynomial.
    /// - `right_coeffs`: The coefficients for the right polynomial.
    /// - `output_coeffs`: The coefficients for the output polynomial.
    /// - `modulus`: The modulus for the field elements.
    pub fn add_constraint(&mut self, left_coeffs: &[(usize, FieldElement)], right_coeffs: &[(usize, FieldElement)], output_coeffs: &[(usize, FieldElement)], modulus: &BigInt) {
        self.qap.add_constraint(left_coeffs, right_coeffs, output_coeffs, modulus);
    }

    /// Generates a witness based on the variable values.
    ///
    /// # Returns
    /// - `Vec<FieldElement>`: The generated witness.
    pub fn generate_witness(&self) -> Vec<FieldElement> {
        self.variables.iter().map(|var| var.value.clone()).collect()
    }

    /// Generates a proof based on the current constraints and witness.
    ///
    /// # Parameters
    /// - `witness`: A vector of `FieldElement` representing the witness.
    ///
    /// # Returns
    /// - `Proof`: The generated proof.
    pub fn generate_proof(&self, witness: &Vec<FieldElement>) -> Proof {
        Proof::generate_proof(self, witness)
    }

    /// Evaluates the QAP with the current witness.
    ///
    /// # Returns
    /// - `BigInt`: The result of the evaluation.
    pub fn evaluate_qap(&self) -> BigInt {
        let witness = self.generate_witness();
        let result = self.qap.evaluate(&witness);
        result.get_value()
    }

    /// Adds a variable and returns its index.
    ///
    /// # Parameters
    /// - `value`: The value of the variable as a `FieldElement`.
    ///
    /// # Returns
    /// - `usize`: The index of the added variable.
    pub fn add_variable(&mut self, value: FieldElement) -> usize {
        let index = self.variables.len();
        self.variables.push(Variable { index, value });
        index
    }

    /// Saves the R1CS to a binary file.
    ///
    /// # Parameters
    /// - `filename`: The name of the file to save the R1CS to.
    pub fn save_to_binary(&self, filename: &str) {
        let mut file = File::create(filename).expect("Could not create proof file");
        let encoded: Vec<u8> = bincode::serialize(&self).expect("Failed to serialize proof");
        file.write_all(&encoded).expect("Failed to write proof to file");
    }

    /// Loads the R1CS from a binary file.
    ///
    /// # Parameters
    /// - `filename`: The name of the file to load the R1CS from.
    ///
    /// # Returns
    /// - `Self`: The loaded R1CS instance.
    pub fn load_from_binary(filename: &str) -> Self {
        let file = File::open(filename).expect("Could not open file");
        let r1cs: R1CS = bincode::deserialize_from(file).expect("Failed to deserialize R1CS");
        r1cs
    }

    /// Verifies the witness against the R1CS constraints.
    ///
    /// # Parameters
    /// - `witness`: A slice of `FieldElement` representing the witness.
    ///
    /// # Returns
    /// - `bool`: `true` if the witness satisfies all constraints, otherwise `false`.
    pub fn verify_witness(&self, witness: &[FieldElement]) -> bool {
        for constraint in &self.constraints {
            let mut left_eval = FieldElement::new(BigInt::zero());
            let mut right_eval = FieldElement::new(BigInt::zero());

            // Evaluate the left side of the constraint
            for (var_index, coeff) in &constraint.left {
                let var_value = &witness[var_index.index];
                left_eval = left_eval + (var_value.clone() * coeff);
            }

            // Evaluate the right side of the constraint
            for (var_index, coeff) in &constraint.right {
                let var_value = &witness[var_index.index];
                right_eval = right_eval + (var_value.clone() * coeff);
            }

            // Evaluate the output side of the constraint
            let mut output_eval = FieldElement::new(BigInt::zero());
            for (var_index, coeff) in &constraint.output {
                let var_value = &witness[var_index.index];
                output_eval = output_eval + (var_value.clone() * coeff);
            }

            // Check if the constraint is satisfied
            if left_eval != right_eval || right_eval != output_eval {
                return false;
            }
        }
        true
    }
}