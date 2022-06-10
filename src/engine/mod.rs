use std::collections::HashMap;

use bot::Bot;
use ruscii::app::{App, Config, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use state::{GameCell, GameState, BOTS_STARTING_ENERGY, MAP_HEIGHT, MAP_WIDTH};

use self::bot::ColorConfig;
use self::utils::direction::Direction;

pub mod action;
pub mod bot;
pub mod resource;
pub mod state;
pub mod utils;

pub fn run_app(bots: Vec<ColorConfig>) {
    let mut app = App::config(Config::new().fps(2));

    let mut fps_counter = FPSCounter::new();
    let mut state = GameState::new(bots);

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        fps_counter.update();
        state.update();

        let mut pencil = Pencil::new(window.canvas_mut());

        pencil
            .draw_text(
                &format!("FPS: {}", fps_counter.count()),
                Vec2::xy(0 as usize, 0 as usize),
            )
            .draw_text("Press 'Q' or 'Esc' for exit", Vec2::y(2 as usize))
            .set_origin(Vec2::xy(1 as usize, 3 as usize))
            .set_foreground(Color::Grey)
            .draw_rect(
                &RectCharset::double_lines(),
                Vec2::xy(-1 as isize, -1 as isize),
                Vec2::xy(MAP_WIDTH + 2, MAP_HEIGHT + 2),
            );

        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                if let GameCell::Bot(bot) = &state.map[x][y] {
                    pencil.set_foreground(bot.color);
                    pencil.draw_char(
                        format!("{}", bot.energy).as_str().chars().next().unwrap(),
                        Vec2::xy(x, MAP_HEIGHT - 1 - y),
                    );
                    match bot.shield_direction {
                        Direction::Down => pencil.draw_char('ˉ', Vec2::xy(x, MAP_HEIGHT - 2 - y)),
                        Direction::Up => pencil.draw_char('_', Vec2::xy(x, MAP_HEIGHT - y)),
                        Direction::Left => {
                            pencil.draw_char('|', Vec2::xy(x - 1, MAP_HEIGHT - 1 - y))
                        }
                        Direction::Right => {
                            pencil.draw_char('|', Vec2::xy(x + 1, MAP_HEIGHT - 1 - y))
                        }
                    };
                } else if let GameCell::Resource(resource) = &state.map[x][y] {
                    pencil.set_foreground(Color::White);
                    pencil.draw_center_text(
                        format!("{}", resource.energy_gain).as_str(),
                        Vec2::xy(x, MAP_HEIGHT - 1 - y),
                    );
                }
            }
        }
    });
}
