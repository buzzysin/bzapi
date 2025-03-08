use bz_partial::Partial; // Assuming your macro is in bz_partial

#[derive(Partial)]
struct MyStruct {
    field1: i32,
    field2: String,
}

fn main() {
    let _ = MyStructPartial {
        field1: Some(42),
        field2: Some("hello".to_string()),
    };
}