mod app;
mod game;
use crate::app::App;

fn main() -> iced::Result {
    iced::run("Game of Life", App::update, App::view).unwrap();
    Ok(())
}
