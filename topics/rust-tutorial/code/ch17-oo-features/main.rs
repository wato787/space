trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: {}", self.label);
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![Box::new(Button {
            label: String::from("OK"),
        })],
    };

    screen.run();
}
