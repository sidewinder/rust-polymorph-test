struct Sheep {
    naked: bool,
    name: &'static str,
}
struct Duck {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

impl Duck {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

impl Animal for Duck {
    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "Quak Quak?"
        } else {
            "Quak Quak!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // Type annotation is necessary in this case.
    let dolly: Sheep = Sheep {
        name: "Dolly",
        naked: false,
    };
    let donald: Duck = Duck {
        name: "Donald",
        naked: false,
    };
    // TODO ^ Try removing the type annotations.

    let mut vec: Vec<Box<Animal>> = Vec::new();
    vec.push(Box::new(dolly));
    vec.push(Box::new(donald));

    for animal in vec.iter() {
        animal.talk();
        //animal.shear();
        animal.talk();    
    }
    /*dolly.talk();
    dolly.shear();
    dolly.talk();
    
    donald.talk();
    donald.shear();
    donald.talk();*/
}
