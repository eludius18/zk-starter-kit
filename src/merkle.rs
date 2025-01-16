use num_bigint::BigInt;
use crate::field::FieldElement;

/// Represents a Merkle Tree with a root and leaves.
pub struct MerkleTree {
    /// The root of the Merkle Tree.
    pub root: BigInt,
    /// The leaves of the Merkle Tree.
    pub leaves: Vec<BigInt>,
}

impl MerkleTree {
    /// Creates a new Merkle Tree from a list of leaves.
    ///
    /// # Parameters
    /// - `leaves`: A vector of `BigInt` representing the leaves.
    ///
    /// # Returns
    /// - `Self`: A new instance of the `MerkleTree` struct.
    pub fn new(leaves: Vec<BigInt>) -> Self {
        let root = MerkleTree::compute_root(&leaves);
        MerkleTree {
            root,
            leaves,
        }
    }

    /// Computes the Merkle path for a given leaf index.
    ///
    /// # Parameters
    /// - `index`: The index of the leaf.
    ///
    /// # Returns
    /// - `Vec<(BigInt, bool)>`: The Merkle path as a vector of tuples containing the sibling hash and a boolean indicating if the current node is a left sibling.
    pub fn merkle_path(&self, index: usize) -> Vec<(BigInt, bool)> {
        let mut path = Vec::new();
        let mut current_index = index;
        let mut nodes = self.leaves.clone();

        while nodes.len() > 1 {
            let next_level: Vec<BigInt> = nodes
                .chunks(2)
                .map(|chunk| {
                    if chunk.len() == 2 {
                        MerkleTree::hash(&chunk[0], &chunk[1])
                    } else {
                        chunk[0].clone() // Handle last single node in an odd-numbered level
                    }
                })
                .collect();

            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < nodes.len() {
                path.push((nodes[sibling_index].clone(), current_index % 2 == 0));
            }

            current_index /= 2;
            nodes = next_level;
        }

        path
    }

    /// Computes the root of the Merkle Tree from the leaves.
    ///
    /// # Parameters
    /// - `leaves`: A reference to a vector of `BigInt` representing the leaves.
    ///
    /// # Returns
    /// - `BigInt`: The root of the Merkle Tree.
    fn compute_root(leaves: &Vec<BigInt>) -> BigInt {
        let mut nodes = leaves.clone();
        while nodes.len() > 1 {
            nodes = nodes.chunks(2).map(|chunk| {
                if chunk.len() == 2 {
                    MerkleTree::hash(&chunk[0], &chunk[1])
                } else {
                    chunk[0].clone()
                }
            }).collect();
        }
        nodes[0].clone()
    }

    /// A simple hashing mechanism combining two BigInt values.
    ///
    /// # Parameters
    /// - `left`: The left `BigInt` value.
    /// - `right`: The right `BigInt` value.
    ///
    /// # Returns
    /// - `BigInt`: The hash of the two values.
    pub fn hash(left: &BigInt, right: &BigInt) -> BigInt {
        // For simplicity, we will just combine the two values
        // and take the modulo of a large prime number.
        let combined = left + right; // Simple addition as the "hashing" operation
        combined % BigInt::from(1_000_000_007) // A large prime for modulo
    }

    /// Hash two FieldElements to create a new FieldElement.
    ///
    /// # Parameters
    /// - `a`: The first `FieldElement`.
    /// - `b`: The second `FieldElement`.
    ///
    /// # Returns
    /// - `FieldElement`: The result of the hash.
    pub fn apply_hash(&self, a: &FieldElement, b: &FieldElement) -> FieldElement {
        // Example hash function: (a + b) % modulus
        assert_eq!(a.get_modulus(), b.get_modulus(), "Moduli must match for hashing");
        let new_value = (a.get_value() + b.get_value()) % a.get_modulus(); // Simple addition as a placeholder for hashing
        FieldElement::new(new_value)
    }
}