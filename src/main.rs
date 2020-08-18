trait Worker {

    fn new(name: &'static str, busy: bool) -> Self;

    fn work(&mut self);

    fn show_data(&self);

    fn rest(&mut self);

}

struct Manager {

    name: String,
    busy: bool

}

impl Worker for Manager {

    fn new(name: &'static str, busy: bool) -> Self {
        return Manager { name: name.to_string(), busy }
    }

    fn work(&mut self) {
        println!("Working...");
        self.busy = true;
    }

    fn show_data(&self) {
        println!("Name: {}", self.name);
        println!("Busy: {}", self.busy);
    }

    fn rest(&mut self) {
        println!("Resting...");
        self.busy = false;
    }
}

fn main() {
    let mut manager: Manager = Worker::new("Jhonatan", false);

    manager.work();
    manager.show_data();

    manager.rest();
    manager.show_data();
}