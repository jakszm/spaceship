use iced::mouse;
use iced::widget::canvas::{Geometry, Path};
use iced::widget::{canvas, image, text, row};
use iced::{keyboard, window};
use iced::{
    Subscription, Element, Rectangle, Fill, Renderer,
    Theme, Vector, Point, Color
};

pub fn main() -> iced::Result {
    iced::application(
        "Spaceship game", Space::update, Space::view)
        .subscription(Space::subscription)
        .run()
}

#[derive(Default)]
struct Space{
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Forward,
    Backward,
    RotateLeft,
    RotateRight,
}

impl Space {
    pub fn update(&mut self, message : Message) {
        match message {
            Message::Forward => {
                self.state.spaceship_position_y += 1.0;
                self.state.update();
            }
            Message::Backward => {
                self.state.spaceship_position_x += 1.0;
                self.state.update();
            }
            Message::RotateLeft => {
                self.state.spaceship_rotation -= 1.0;
                self.state.update();
            }
            Message::RotateRight => {
                self.state.spaceship_rotation += 1.0;
                self.state.update();
            }
        }
    }
    
    pub fn subscription(&self) -> Subscription<Message> {
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

    pub fn view(&self) -> Element<Message> {
        canvas(&self.state).width(Fill).height(Fill).into()
    }
}

#[derive(Debug)]
struct State {
    spaceship : image::Handle,
    spaceship_cache : canvas::Cache,
    spaceship_rotation : f32,
    spaceship_position_x : f32,
    spaceship_position_y : f32,
    space_cache : canvas::Cache,


}

impl State {
    pub fn new() -> State{
        State {
            spaceship : image::Handle::from_bytes(
                       include_bytes!("../spaceship1.png").as_slice(),
                   ),
            spaceship_cache : canvas::Cache::default(),
            space_cache : canvas::Cache::default(),
            spaceship_rotation : 0.0,
            spaceship_position_x : 0.0,
            spaceship_position_y : 0.0,
        }
    }
    pub fn update(&mut self) {
        self.spaceship_cache.clear();
    }
}

impl<Message> canvas::Program<Message> for State{
    type State= ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {

        use std::f32::consts::PI;
        let background =
            self.space_cache.draw(renderer, bounds.size(), |frame| {
                frame.fill_rectangle(Point::ORIGIN, frame.size(), Color::BLACK);
                frame.translate(frame.center() - Point::ORIGIN);
            });
        let spaceship = self.spaceship_cache.draw(renderer, bounds.size(), |frame| {
            //let center = frame.center();
            frame.rotate(20.0);
            frame.translate(Vector::new(
                    /*center.x + */self.spaceship_position_x,
                    /*center.y + */self.spaceship_position_y));
            let rotation = self.spaceship_rotation * (2.0 * PI / 60.0); 
            frame.draw_image(
                Rectangle::with_radius(35.0),
                canvas::Image::new(&self.spaceship).rotation(rotation),
            );

        });
        vec![background, spaceship]
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
