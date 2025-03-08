use bz_partial::Partial;

#[derive(Partial)]
enum MyEnum { // Should fail because Partial only supports structs
    Variant1,
    Variant2,
}

fn main() {}
