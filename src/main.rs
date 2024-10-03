use iced::widget::{canvas, Image};
use iced::keyboard;
use iced::Subscription;
use iced::{Element, Radians, Rotation, Shrink};

struct Spaceship {
    x  : u16,
    y  : u16,
    rotation : Rotation,
}

impl Default for Spaceship {
    fn default() -> Spaceship {
        Spaceship {
            x : 50,
            y : 50,
            rotation : Rotation::Floating{0 : Radians(0.)},
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Message {
    Forward,
    Backward,
    RotateLeft,
    RotateRight,
}

impl Spaceship {
    fn update(&mut self, k : Message) {
        match k {
            Message::Forward => {
                self.x += 1;        
            }
            Message::Backward => {
                self.y += 1;        
            }
            Message::RotateLeft => {
                let mut rot = self.rotation.radians();
                rot -= Radians(0.1);
                self.rotation = Rotation::Floating{0 : rot};        
            }
            Message::RotateRight => {
                let mut rot = self.rotation.radians();
                rot += Radians(0.1);
                self.rotation = Rotation::Floating{0 : rot};        
            }
        }
    }
    
    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_press(|key, _modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match (key, _modifiers) {
                (key::Named::ArrowUp, _) => Some(Message::Forward),
                (key::Named::ArrowDown, _) => Some(Message::Backward),
                (key::Named::ArrowLeft, _) => Some(Message::RotateLeft),
                (key::Named::ArrowRight, _) => Some(Message::RotateRight),
                _ => None,
            }
        })
    }

    fn view(&self) -> Element<Message> {

        container(
            Image::new("spaceship1.png").rotation(self.rotation).width(100).height(100)
        )
        .center(Shrink)
        .style(container::bordered_box)
        .into()
    }
}

pub fn main() -> iced::Result {
    iced::application("Spaceship game", Spaceship::update, Spaceship::view)
        .subscription(Spaceship::subscription)
        .run()
}
