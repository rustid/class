trait NoiseMaker {
    fn make_noise(&self) -> &'static str;
}

struct Dog;
impl NoiseMaker for Dog {
    fn make_noise(&self) -> &'static str {
        "Woof!"
    }
}

struct Cat;
impl NoiseMaker for Cat {
    fn make_noise(&self) -> &'static str {
        "Meow!"
    }
}

// fn returns_noisemaker(switch: bool) -> impl NoiseMaker {
fn returns_noisemaker(switch: bool) -> Box<dyn NoiseMaker> {
    if switch {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

fn main() {}
