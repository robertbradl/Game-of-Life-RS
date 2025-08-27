use crate::game;
use array2d::Array2D;
use iced::{
    Color, Length, Padding,
    widget::{Button, Column, Container, Row},
};
use rand_distr::{Distribution, Uniform};

#[derive(Debug, Clone)]
pub enum Message {
    Future,
    ToggleCell(usize, usize),
    Randomize,
    Clear,
}

pub struct App {
    grid: Array2D<u8>,
}

impl Default for App {
    fn default() -> Self {
        let row_size: usize = 50;
        let col_size: usize = 100;

        Self {
            grid: Array2D::filled_with(0u8, row_size, col_size),
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) {
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
        }
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

        let buttons_column = Column::new().spacing(5).push(
            Row::new()
                .spacing(5)
                .push(Button::new("Future Generation").on_press(Message::Future))
                .push(Button::new("Clear").on_press(Message::Clear))
                .push(Button::new("Randomize").on_press(Message::Randomize)),
        );

        Column::new()
            .spacing(20)
            .push(Container::new(buttons_column).padding(Padding {
                top: 10.0,
                left: 10.0,
                ..Default::default()
            }))
            .push(
                Container::new(grid_column).style(|_| iced::widget::container::Style {
                    border: iced::Border {
                        width: 2.0,
                        color: Color::from_rgba(128.0, 128.0, 128.0, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
            .into()
    }
}
