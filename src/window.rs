use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

pub struct MyApp {
    button_state: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ButtonClicked,
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            button_state: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("ArmoredCAD")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonClicked => {
                println!("Hello, world!");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new("Click the button!"))
            .push(Button::new(&mut self.button_state, Text::new("Click me!")).on_press(Message::ButtonClicked))
            .into()
    }
}

fn main() {
    let settings = Settings::default();
    match MyApp::run(settings) {
        Ok(_) => {}
        Err(e) => eprintln!("Application error: {:?}", e),
    }
}
