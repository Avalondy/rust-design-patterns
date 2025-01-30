#![allow(unused)]

#[derive(Debug)]
pub struct Foo {
    bar: String,
}

impl Foo {
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new() -> Self {
        FooBuilder::default()
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        self.bar = bar;
        self
    }

    pub fn build(self) -> Foo {
        Foo { bar: self.bar }
    }
}

fn main() {
    let foo = FooBuilder::new().name(String::from("Y")).build();
    println!("{:?}", foo);
}
