use iced::mouse;
use iced::widget::canvas::Geometry;
use iced::widget::{canvas, image};
use iced::keyboard;
use iced::{
    Subscription, Element, Rectangle, Fill, Renderer,
    Theme, Vector, Point, Color, Size
};

use iced::time;
use std::time::{Instant, Duration};
use std::f32::consts::PI;

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

enum Dimension {
    X,
    Y,
}
#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
    Forward,
    Backward,
    RotateLeft,
    RotateRight,
}

impl Space {
    pub fn update(&mut self, message : Message) {
        match message {
            Message::Tick(_instant) => {
                self.state.update();
            }
            Message::Forward => {
                self.state.spaceship.speed_update(1.0);
                self.state.update();
            }
            Message::Backward => {
                self.state.spaceship.speed_update(-1.0);
                self.state.update();
            }
            Message::RotateLeft => {
                self.state.spaceship.rotation -= 2.0 * PI / 20.0;
                self.state.update();
            }
            Message::RotateRight => {
                self.state.spaceship.rotation += 2.0 * PI / 20.0;
                self.state.update();
            }
         }
    }
    
    pub fn subscription(&self) -> Subscription<Message> {
        let tick = time::every(Duration::from_millis(1000 / self.state.spaceship.speed.abs() as u64))
                .map(|_| Message::Tick(Instant::now()));

        fn handle_hotkey(
            key: keyboard::Key,
            _modifiers : keyboard::Modifiers,
        ) -> Option<Message> {
            use keyboard::key;
            match key.as_ref() {
                keyboard::Key::Named(key::Named::ArrowUp) => Some(Message::Forward),
                keyboard::Key::Named(key::Named::ArrowDown) => Some(Message::Backward),
                keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::RotateLeft),
                keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::RotateRight),
                _ => None,
            }
        }
        Subscription::batch(vec![tick, keyboard::on_key_press(handle_hotkey)])
    }

    pub fn view(&self) -> Element<Message> {
        canvas(&self.state).width(Fill).height(Fill).into()
    }
}

#[derive(Debug)]
struct State {
    space_cache : canvas::Cache,
    spaceship : Spaceship,
}

impl State {
    pub fn new() -> State{
        State {
            space_cache : canvas::Cache::default(),
            spaceship : Spaceship::default(),
        }
    }
    pub fn update(&mut self) {
        self.spaceship.calculate_position();
        self.spaceship.cache.clear();
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

        let background =
            self.space_cache.draw(renderer, bounds.size(), |frame| {
                frame.fill_rectangle(Point::ORIGIN, frame.size(), Color::BLACK);
                frame.translate(frame.center() - Point::ORIGIN);
            });
        let spaceship = self.spaceship.cache.draw(renderer, self.spaceship.size, |frame| {
            frame.translate(Vector::new(
                bounds.size().width / 2.0 + self.spaceship.position_x,
                bounds.size().height / 2.0 + self.spaceship.position_y));
            frame.draw_image(
                Rectangle::with_radius(35.0),
                canvas::Image::new(&self.spaceship.handle).rotation(self.spaceship.rotation),
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

#[derive(Debug)]
struct Spaceship {
    handle : image::Handle,
    cache : canvas::Cache,
    rotation : f32,
    position_x : f32,
    position_y : f32,
    speed : f32,
    size : Size,
}

impl Spaceship {
    pub fn new() -> Spaceship {
        Spaceship {
            handle : image::Handle::from_bytes(
                       include_bytes!("../spaceship1.png").as_slice(),
                   ),
            cache : canvas::Cache::default(),
            rotation : 0.0,
            position_x : 0.0,
            position_y : 0.0,
            speed : 1.0,
            size : Size{width: 100.0, height: 100.0},
        }
    }
    pub fn calculate_position(&mut self) {
        println!("x {}, y {}", self.position_x, self.position_y);
        if self.speed > 0.0 {
            self.position_x += f32::sin(self.rotation);
            self.position_y -= f32::cos(self.rotation);

            if self.position_x > 1000.0 {
                self.flip_position(Dimension::X);
            }
            if self.position_y > 1000.0 {
                self.flip_position(Dimension::Y);
            }
        } else {
            self.position_x -= f32::sin(self.rotation);
            self.position_y += f32::cos(self.rotation);
        }
    }
    pub fn flip_position(&mut self, d : Dimension) {
        match d {
            Dimension::X => self.position_x = 0.0,
            Dimension::Y => self.position_y = 0.0,
        }
    }
    pub fn speed_update(&mut self, val: f32) {
        self.speed += val;
        if self.speed == 0.0 {
            self.speed += val;
        }
    }
}

impl Default for Spaceship {
    fn default() -> Self {
        Self::new()
    }
}

