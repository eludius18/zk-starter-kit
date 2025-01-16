use std::ops::AddAssign;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::ops::{Add, Mul};
use serde::{Deserialize, Serialize};

/// Represents an element in a finite field.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct FieldElement {
    /// The value of the field element.
    value: BigInt,
    /// The modulus of the field.
    modulus: BigInt,
}

impl FieldElement {
    /// Creates a new field element with a default modulus.
    ///
    /// # Parameters
    /// - `value`: The value of the field element.
    ///
    /// # Returns
    /// - `Self`: A new instance of the `FieldElement` struct.
    pub fn new(value: BigInt) -> Self {
        let default_modulus = BigInt::from(1_000_000_007);
        let normalized_value = value.clone() % &default_modulus;
        FieldElement { value: normalized_value, modulus: default_modulus }
    }

    /// Gets the underlying value of the field element.
    ///
    /// # Returns
    /// - `BigInt`: The value of the field element.
    pub fn get_value(&self) -> BigInt {
        self.value.clone()
    }

    /// Gets the modulus of the field element.
    ///
    /// # Returns
    /// - `&BigInt`: A reference to the modulus.
    pub fn get_modulus(&self) -> &BigInt {
        &self.modulus
    }

    /// Adds two field elements.
    ///
    /// # Parameters
    /// - `other`: The other field element to add.
    ///
    /// # Returns
    /// - `FieldElement`: The result of the addition.
    pub fn add(&self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.modulus, other.modulus);
        FieldElement::new(self.value.clone() + other.value.clone())
    }

    /// Subtracts one field element from another.
    ///
    /// # Parameters
    /// - `other`: The other field element to subtract.
    ///
    /// # Returns
    /// - `FieldElement`: The result of the subtraction.
    pub fn sub(&self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.modulus, other.modulus);
        FieldElement::new(self.value.clone() - other.value.clone())
    }

    /// Multiplies two field elements.
    ///
    /// # Parameters
    /// - `other`: The other field element to multiply.
    ///
    /// # Returns
    /// - `FieldElement`: The result of the multiplication.
    pub fn mul(&self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.modulus, other.modulus);
        FieldElement::new(&self.value * &other.value % &self.modulus)
    }

    /// Computes the multiplicative inverse of the field element.
    ///
    /// # Returns
    /// - `FieldElement`: The multiplicative inverse.
    ///
    /// # Panics
    /// - If the inverse does not exist.
    pub fn inv(&self) -> FieldElement {
        let (gcd, x, _) = self.extended_gcd(&self.value, &self.modulus);
        if gcd != BigInt::one() {
            panic!("Inverse does not exist");
        }
        // Normalize the inverse to be positive
        let normalized_inverse = x % &self.modulus;
        FieldElement::new(normalized_inverse)
    }

    /// Negates the field element.
    ///
    /// # Returns
    /// - `FieldElement`: The negated field element.
    pub fn negate(&self) -> FieldElement {
        FieldElement::new(&self.modulus - &self.value)
    }

    /// Computes the extended GCD of two numbers.
    ///
    /// # Parameters
    /// - `a`: The first number.
    /// - `b`: The second number.
    ///
    /// # Returns
    /// - `(BigInt, BigInt, BigInt)`: The GCD and the coefficients of Bézout's identity.
    fn extended_gcd(&self, a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
        if *b == BigInt::zero() {
            return (a.clone(), BigInt::one(), BigInt::zero());
        }
        let (gcd, x1, y1) = self.extended_gcd(b, &(a % b));
        let x = y1.clone();
        let y = x1 - (a / b) * &y1;
        (gcd, x, y)
    }
}

// Implement AddAssign for FieldElement
impl AddAssign for FieldElement {
    /// Adds another field element to this one, in place.
    ///
    /// # Parameters
    /// - `other`: The other field element to add.
    fn add_assign(&mut self, other: FieldElement) {
        assert_eq!(self.modulus, other.modulus, "Moduli must match for addition");
        self.value = (self.value.clone() + other.value) % &self.modulus; // Perform addition and normalize
    }
}

// Implement Add for FieldElement
impl Add for FieldElement {
    type Output = FieldElement;

    /// Adds two field elements.
    ///
    /// # Parameters
    /// - `other`: The other field element to add.
    ///
    /// # Returns
    /// - `FieldElement`: The result of the addition.
    fn add(self, other: FieldElement) -> FieldElement {
        let mut result = self.clone(); // Start with a copy of self
        result += other; // Use the AddAssign implementation
        result
    }
}

// Implementing Add for references
impl Add for &FieldElement {
    type Output = FieldElement;

    /// Adds two field elements.
    ///
    /// # Parameters
    /// - `other`: The other field element to add.
    ///
    /// # Returns
    /// - `FieldElement`: The result of the addition.
    fn add(self, other: &FieldElement) -> FieldElement {
        assert_eq!(self.modulus, other.modulus, "Moduli must match for addition");
        FieldElement::new((self.value.clone() + other.value.clone()) % &self.modulus)
    }
}

// Implementing Mul trait for FieldElement
impl Mul<&BigInt> for FieldElement {
    type Output = FieldElement;

    /// Multiplies a field element by a BigInt.
    ///
    /// # Parameters
    /// - `rhs`: The BigInt to multiply by.
    ///
    /// # Returns
    /// - `FieldElement`: The result of the multiplication.
    fn mul(self, rhs: &BigInt) -> FieldElement {
        FieldElement::new((self.value.clone() * rhs) % &self.modulus) // Perform multiplication and normalize
    }
}

// Implementing Mul trait for FieldElement
impl Mul<BigInt> for FieldElement {
    type Output = FieldElement;

    /// Multiplies a field element by a BigInt.
    ///
    /// # Parameters
    /// - `rhs`: The BigInt to multiply by.
    ///
    /// # Returns
    /// - `FieldElement`: The result of the multiplication.
    fn mul(self, rhs: BigInt) -> FieldElement {
        self * &rhs // Delegate to the implementation that takes a reference
    }
}