trait Pet {
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}

struct Cat;

impl Pet for Dog {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Pet for Cat {
    fn name(&self) -> &str {
        "The cat" // No name, cats won't respond to it anyway.
    }
}

fn greet<P: Pet>(pet: &P) {
    println!("Who's a cutie? {} is!", pet.name());
}

fn main() {
    let fido = Dog {
        name: "Fido".to_string(),
    };
    greet(&fido);

    let floof = Cat;
    greet(&floof);
}
