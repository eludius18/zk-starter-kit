use num_bigint::BigInt;
use crate::r1cs::R1CS;
use crate::field::FieldElement;
use crate::proof::Proof;

/// Represents a gate in the circuit.
pub enum Gate {
    /// Addition gate: input1, input2, output
    Add(usize, usize, usize),
    /// Multiplication gate: input1, input2, output
    Mul(usize, usize, usize),
}

/// Represents a circuit with inputs, gates, and outputs.
pub struct Circuit {
    /// The inputs to the circuit, stored as FieldElements.
    inputs: Vec<FieldElement>,
    /// The gates in the circuit.
    gates: Vec<Gate>,
    /// The outputs of the circuit, stored as FieldElements.
    outputs: Vec<FieldElement>,
    /// The modulus used for FieldElements.
    modulus: BigInt,
}

impl Circuit {
    /// Creates a new circuit with a default modulus.
    ///
    /// # Returns
    /// - `Self`: A new instance of the `Circuit` struct.
    pub fn new() -> Self {
        let default_modulus = BigInt::from(1_000_000_007); // Default modulus
        Circuit {
            inputs: Vec::new(),
            gates: Vec::new(),
            outputs: Vec::new(),
            modulus: default_modulus,
        }
    }

    /// Adds an input to the circuit.
    ///
    /// # Parameters
    /// - `value`: The input value as a `FieldElement`.
    ///
    /// # Returns
    /// - `usize`: The index of the added input.
    pub fn add_input(&mut self, value: FieldElement) -> usize {
        let index = self.inputs.len();
        self.inputs.push(value);
        index
    }

    /// Adds a gate to the circuit.
    ///
    /// # Parameters
    /// - `gate`: The gate to add, represented as a `Gate` enum.
    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    /// Sets an output for the circuit.
    ///
    /// # Parameters
    /// - `value`: The output value as a `FieldElement`.
    pub fn set_output(&mut self, value: FieldElement) {
        self.outputs.push(value);
    }
}