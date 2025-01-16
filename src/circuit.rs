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

    /// Retrieves an input value by index, if it exists.
    ///
    /// # Parameters
    /// - `index`: The index of the input.
    ///
    /// # Returns
    /// - `Option<&FieldElement>`: The input value if it exists, otherwise `None`.
    pub fn get_input(&self, index: usize) -> Option<&FieldElement> {
        self.inputs.get(index)
    }

    /// Generates the proof and checks constraint satisfaction, then saves it to a binary file.
    ///
    /// # Parameters
    /// - `proof_file`: The name of the file to save the proof to.
    pub fn generate_proof(&self, proof_file: &str) {
        // Ensure inputs are added before generating proof
        if self.inputs.is_empty() {
            panic!("No inputs available to generate proof.");
        }

        let mut r1cs = R1CS::new();

        // Add variables to R1CS
        for input in &self.inputs {
            r1cs.add_variable(input.clone()); // input is of type FieldElement
        }

        // Process each gate and add constraints to R1CS
        for gate in &self.gates {
            match gate {
                Gate::Add(a, b, output) => {
                    r1cs.add_constraint(
                        &[
                            (r1cs.variables[*a].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &[
                            (r1cs.variables[*b].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &[
                            (r1cs.variables[*output].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &self.modulus, // Pass modulus dynamically
                    );
                },
                Gate::Mul(a, b, output) => {
                    r1cs.add_constraint(
                        &[
                            (r1cs.variables[*a].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &[
                            (r1cs.variables[*b].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &[
                            (r1cs.variables[*output].index, FieldElement::new(BigInt::from(1))), // Extract index
                        ],
                        &self.modulus, // Pass modulus dynamically
                    );
                },
            }
        }

        // Save the R1CS to a binary file
        r1cs.save_to_binary("r1cs_file.bin");

        // Generate the witness and proof
        let witness = r1cs.generate_witness();
        let proof = r1cs.generate_proof(&witness);

        // Save the proof to a specified file
        proof.save_to_binary(proof_file).expect("failed to save the proof");
    }

    /// Verifies the proof by reading from a binary file.
    ///
    /// # Parameters
    /// - `proof_file`: The name of the file to read the proof from.
    ///
    /// # Returns
    /// - `bool`: `true` if the proof is valid, otherwise `false`.
    pub fn verify_proof(&self, proof_file: &str) -> bool {
        let proof_data = std::fs::read(proof_file).expect("Could not read proof file");

        let proof = bincode::deserialize::<Proof>(&proof_data).expect("Failed to deserialize proof");

        // Ensure that witness is Vec<FieldElement> and not Vec<BigInt>
        let witness: Vec<FieldElement> = proof.witness.iter()
            .map(|value| FieldElement::new(value.clone()))
            .collect();

        let r1cs = R1CS::load_from_binary("r1cs_file.bin");

        let is_valid = r1cs.verify_witness(&witness);

        println!("Proof verification result: {}", is_valid);
        is_valid
    }
}