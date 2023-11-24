use std::fmt::Debug;

// Define a trait PrintInfo
trait PrintInfo {
    fn print_info(&self);
}

// Blanket implementation for any type T that implements Debug
impl<T: Debug> PrintInfo for T {
    fn print_info(&self) {
        println!(
            "Type: {:?}, Debug Info: {:?}",
            std::any::type_name::<T>(),
            self
        );
    }
}

#[derive(Debug)]
struct CustomStruct {
    field1: i32,
    field2: String,
}

fn main() {
    // Example with i32
    let number = 42;
    number.print_info();

    // Example with String
    let text = String::from("Hello, Rust!");
    text.print_info();

    // Example with a custom struct
    let custom_instance = CustomStruct {
        field1: 123,
        field2: String::from("Rust is awesome!"),
    };
    custom_instance.print_info();
}
