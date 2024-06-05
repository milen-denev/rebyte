use rebyte::{memory_handler::MemoryHandler, rebyte::{get_struct_bytes, ReByte}};

#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

fn main() { 
    let person = Person {
        name: "Test".into(),
        age: 100
    };

    let bytes = get_struct_bytes::<Person>(person);

    let mut rebyte = ReByte::<Person>::default();
    rebyte.allocate();
    rebyte.set_value_bytes(bytes);
    let value = rebyte.get_value();

    println!("{:?}", value);

    drop(rebyte);
}