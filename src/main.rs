mod app;
mod game;
use crate::app::App;

fn main() -> iced::Result {
    iced::application("Game of Life", App::update, App::view)
        .subscription(App::subscription)
        .run()
        .unwrap();
    Ok(())
}
