use crate::game;
use array2d::Array2D;
use iced::{
    Alignment::Center,
    Color, Length, Padding, Subscription, Task,
    time::{self, Duration},
    widget::{Button, Column, Container, Row, Slider, Toggler, column, row, text},
};
use rand_distr::{Distribution, Uniform};

#[derive(Debug, Clone)]
pub enum Message {
    Future,
    ToggleCell(usize, usize),
    Randomize,
    Clear,
    ToggleAutomatic(bool),
    SliderChange(f64),
}

pub struct App {
    grid: Array2D<u8>,
    is_running: bool,
    speed: f64,
}

impl Default for App {
    fn default() -> Self {
        let row_size: usize = 100;
        let col_size: usize = 200;

        Self {
            grid: Array2D::filled_with(0u8, row_size, col_size),
            is_running: false,
            speed: 1.0,
        }
    }
}

impl App {
    pub fn subscription(state: &App) -> Subscription<Message> {
        if state.is_running {
            time::every(Duration::from_secs_f64(state.speed)).map(|_| Message::Future)
        } else {
            Subscription::none()
        }
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Future => self.grid = game::future_generation(&self.grid),
            Message::Clear => {
                self.grid = Array2D::filled_with(0u8, self.grid.num_rows(), self.grid.num_columns())
            }
            Message::ToggleCell(row, col) => {
                if let Some(value) = self.grid.get_mut(row, col) {
                    *value = if *value == 0u8 { 1u8 } else { 0u8 };
                }
            }
            Message::Randomize => {
                let mut rng = rand::rng();
                let dist =
                    Uniform::new_inclusive(1, 10).expect("Failure to create random distribution!");
                for index in self.grid.indices_row_major() {
                    let num = dist.sample(&mut rng);
                    if num < 4 {
                        self.grid.set(index.0, index.1, 1u8).unwrap();
                    } else {
                        self.grid.set(index.0, index.1, 0u8).unwrap();
                    }
                }
            }
            Message::ToggleAutomatic(is_checked) => {
                self.is_running = is_checked;
            }
            Message::SliderChange(value) => self.speed = value,
        }
        Task::none()
    }

    pub fn view(&self) -> iced::Element<Message> {
        let mut grid_column = Column::new().width(Length::Fill).height(Length::Fill);
        for (row_idx, row) in self.grid.rows_iter().enumerate() {
            let mut iced_row = Row::new();

            for (col_idx, &cell) in row.enumerate() {
                let color = if cell == 0u8 {
                    Color::from_rgb(0.0, 0.0, 0.0)
                } else {
                    Color::from_rgb(255.0, 255.0, 255.0)
                };

                let cell_button = Button::new("")
                    .style(move |_, _| iced::widget::button::Style {
                        background: Some(color.into()),
                        border: iced::Border {
                            width: 0.5,
                            color: Color::from_rgba(128.0, 128.0, 128.0, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(Message::ToggleCell(row_idx, col_idx));

                iced_row = iced_row.push(cell_button);
            }

            grid_column = grid_column.push(iced_row);
        }

        let speed_slider: Column<'_, Message> = column![
            row![text(format!("Iteration speed: {:.2} sec", self.speed))]
                .align_y(Center)
                .spacing(5),
            row![
                Slider::new(0.1..=1.0f64, self.speed, Message::SliderChange)
                    .step(0.05f64)
                    .width(175)
            ]
            .align_y(Center)
            .spacing(5),
        ]
        .spacing(5)
        .align_x(Center);

        let button_row: Row<'_, Message> = row![
            Button::new("Future Generation").on_press(Message::Future),
            Button::new("Clear").on_press(Message::Clear),
            Button::new("Randomize").on_press(Message::Randomize),
            column![
                row![text("Run continuously")].align_y(Center),
                row![
                    Toggler::new(self.is_running)
                        .label("")
                        .on_toggle(Message::ToggleAutomatic)
                ]
                .align_y(Center)
            ]
            .align_x(Center),
        ]
        .align_y(Center)
        .spacing(10);

        let top_bar: Column<'_, Message> = Column::new()
            .padding(10)
            .push(row![button_row, speed_slider].spacing(15).align_y(Center));

        Column::new()
            .align_x(Center)
            .push(Container::new(top_bar).padding(Padding::new(10f32).bottom(0)))
            .push(Container::new(grid_column).padding(Padding::new(20f32)))
            .into()
    }
}
