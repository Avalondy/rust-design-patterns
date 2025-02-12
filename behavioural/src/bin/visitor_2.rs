#![allow(unused)]

/// Visitor can visit one type, do conversions, and output another type.
trait Visitor {
    type Value;

    /// Visits a vector of integers and outputs a desired type
    fn visit_vec(&self, v: Vec<i32>) -> Self::Value;
}

/// A struct of two integer values.
#[derive(Debug, Default)]
struct TwoValuesStruct {
    a: i32,
    b: i32,
}

/// Visitor implementation for a struct of two values.
impl Visitor for TwoValuesStruct {
    type Value = Self;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        Self { a: v[0], b: v[1] }
    }
}

/// A struct of an integer array with two values.
#[derive(Debug, Default)]
struct TwoValuesArray {
    ab: [i32; 2],
}

/// Visitor implementation for a struct of values array.
impl Visitor for TwoValuesArray {
    type Value = Self;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        Self { ab: [v[0], v[1]] }
    }
}

/// `Deserializer` trait defines methods that can parse either a string or
/// a vector, it accepts a visitor which knows how to construct a new object
/// of a desired type (in our case, `TwoValuesArray` and `TwoValuesStruct`).
trait Deserializer<V: Visitor> {
    fn create(visitor: V) -> Self;

    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str> {
        Err("parse_str is unimplemented")
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<V::Value, &'static str> {
        Err("parse_vec is unimplemented")
    }
}

struct StringDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for StringDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_str(&self, input: &str) -> Result<<V as Visitor>::Value, &'static str> {
        let input_vec = input
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(self.visitor.visit_vec(input_vec))
    }
}

struct VecDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for VecDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<<V as Visitor>::Value, &'static str> {
        Ok(self.visitor.visit_vec(input))
    }
}

fn main() {
    let deserializer = StringDeserializer::create(TwoValuesStruct::default());
    let result = deserializer.parse_str("123 456");
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValuesStruct::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValuesArray::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    println!(
        "Error: {}",
        deserializer.parse_str("123 456").err().unwrap()
    )
}
