use num_bigint::BigInt;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use num_traits::Zero;
use crate::field::FieldElement;

/// Represents a variable in the QAP.
#[derive(Clone, Serialize, Deserialize)]
pub struct Variable {
    /// The index of the variable.
    pub index: usize,
    /// The value of the variable.
    pub value: BigInt,
}

/// Represents a polynomial with coefficients keyed by variable index.
#[derive(Serialize, Deserialize)]
pub struct Polynomial {
    /// The coefficients of the polynomial.
    coefficients: HashMap<usize, FieldElement>,
}

/// Represents a Quadratic Arithmetic Program (QAP).
#[derive(Serialize, Deserialize)]
pub struct QAP {
    /// The left polynomial.
    pub left: Polynomial,
    /// The right polynomial.
    pub right: Polynomial,
    /// The output polynomial.
    pub output: Polynomial,
}

impl QAP {
    /// Creates a new QAP instance.
    ///
    /// # Returns
    /// - `Self`: A new instance of the `QAP` struct.
    pub fn new() -> Self {
        QAP {
            left: Polynomial::new(),
            right: Polynomial::new(),
            output: Polynomial::new(),
        }
    }

    /// Adds a constraint to the QAP.
    ///
    /// # Parameters
    /// - `left_coeffs`: The coefficients for the left polynomial.
    /// - `right_coeffs`: The coefficients for the right polynomial.
    /// - `output_coeffs`: The coefficients for the output polynomial.
    /// - `_modulus`: The modulus for the field elements.
    pub fn add_constraint(&mut self, left_coeffs: &[(usize, FieldElement)], right_coeffs: &[(usize, FieldElement)], output_coeffs: &[(usize, FieldElement)], _modulus: &BigInt) {
        for (index, coeff) in left_coeffs {
            *self.left.coefficients.entry(*index).or_insert(FieldElement::new(BigInt::zero())) += coeff.clone();
        }
        for (index, coeff) in right_coeffs {
            *self.right.coefficients.entry(*index).or_insert(FieldElement::new(BigInt::zero())) += coeff.clone();
        }
        for (index, coeff) in output_coeffs {
            *self.output.coefficients.entry(*index).or_insert(FieldElement::new(BigInt::zero())) += coeff.clone();
        }
    }

    /// Evaluates the QAP with a given assignment.
    ///
    /// # Parameters
    /// - `assignment`: A vector of `FieldElement` representing the assignment.
    ///
    /// # Returns
    /// - `FieldElement`: The result of the evaluation.
    pub fn evaluate(&self, assignment: &Vec<FieldElement>) -> FieldElement {
        let left_eval = self.left.evaluate(assignment);
        let right_eval = self.right.evaluate(assignment);
        let output_eval = self.output.evaluate(assignment);

        // Return the evaluation result: left * right - output
        left_eval.mul(&right_eval).sub(&output_eval)
    }
}

impl Polynomial {
    /// Creates a new polynomial instance.
    ///
    /// # Returns
    /// - `Self`: A new instance of the `Polynomial` struct.
    pub fn new() -> Self {
        Polynomial { coefficients: HashMap::new() }
    }

    /// Adds a term to the polynomial.
    ///
    /// # Parameters
    /// - `index`: The index of the variable.
    /// - `coefficient`: The coefficient of the term.
    pub fn add_term(&mut self, index: usize, coefficient: FieldElement) {
        self.coefficients.insert(index, coefficient);
    }

    /// Evaluates the polynomial with a given assignment.
    ///
    /// # Parameters
    /// - `assignment`: A vector of `FieldElement` representing the assignment.
    ///
    /// # Returns
    /// - `FieldElement`: The result of the evaluation.
    pub fn evaluate(&self, assignment: &Vec<FieldElement>) -> FieldElement {
        let mut result = FieldElement::new(BigInt::zero()); // Use the same modulus
        for (index, coefficient) in &self.coefficients {
            result = result.add(&coefficient.mul(&assignment[*index]));
        }
        result
    }

    /// Perform Lagrange interpolation to find a polynomial that passes through all given points.
    ///
    /// # Parameters
    /// - `points`: A slice of tuples representing the points (x, y).
    /// - `_modulus`: The modulus for the field elements.
    ///
    /// # Returns
    /// - `Polynomial`: The interpolated polynomial.
    pub fn interpolate(points: &[(FieldElement, FieldElement)], _modulus: &BigInt) -> Polynomial {
        let mut result = Polynomial::new();

        for (i, &(ref x_i, ref y_i)) in points.iter().enumerate() {
            // Start with y_i
            let mut term = vec![(0, y_i.clone())];

            // Compute the Lagrange basis polynomial L_i(x)
            for (j, &(ref x_j, _)) in points.iter().enumerate() {
                if i != j {
                    let denom = x_i.sub(x_j).inv();
                    let negated_x_j = x_j.negate();
                    let coeff = denom.mul(&negated_x_j);

                    term.push((1, denom)); // L_i(x) = product (x - x_j) / (x_i - x_j)

                    for k in 0..term.len() {
                        term[k] = (term[k].0, term[k].1.mul(&coeff));
                    }
                }
            }

            // Add L_i(x) * y_i to the result
            for (index, coeff) in term.iter() {
                *result.coefficients.entry(*index).or_insert(FieldElement::new(BigInt::zero())) += coeff.clone();
            }
        }

        result
    }
}