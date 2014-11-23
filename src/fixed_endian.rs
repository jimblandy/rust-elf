//! The `elf::fixed_endian` module provides fixed-width integer types that store
//! their values in a specific byte ordering.
//!
//! These are valuable when treating file bytes as a structure: you can write
//! out the format of the bytes in the stream as a struct, and then borrow a
//! slice of the bytes as an instance of the struct.

trait FixedEndian<T: Int> {
    fn get(&self) -> T;
    fn set(&self, v: T);
}

struct BigEndian<T: Int> {
    encoded: T;
}

impl<T: Int> FixedEndian<T> for BigEndian<T> {
    fn get(&self) -> T { encoded.from_be() }
    fn set(&self, v: T) { encoded = v.to_be(); }
}

struct LittleEndian<T: Int> {
    encoded: T;
}

impl<T: Int> FixedEndian<T> for LittleEndian<T> {
    fn get(&self) -> T { encoded.from_le() }
    fn set(&self, v: T) { encoded = v.to_le(); }
}

