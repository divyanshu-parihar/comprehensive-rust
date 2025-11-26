#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: i8,
}

impl Person {
    fn new(name: String, age: i8) -> Self {
        Self { name, age }
    }
    fn name(&mut self, name: String) {
        self.name = (|| -> String { name + &String::from("Parihar") })();
    }
}
