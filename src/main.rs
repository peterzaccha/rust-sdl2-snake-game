mod grid;

use grid::Game;
//export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rwops::RWops;
use sdl2::ttf::Font;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font: &[u8] = include_bytes!("./assets/GothamBook.ttf");
    let font: &Font = &ttf_context
        .load_font_from_rwops(RWops::from_bytes(font).unwrap(), 128)
        .unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 600, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // fills the canvas with the color we set in `set_draw_color`.
    canvas.clear();

    let mut game = Game::new(20, 20);
    game.start();

    // render a surface, and convert it to a texture bound to the canvas

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion {
                    timestamp,
                    window_id,
                    which,
                    mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                } => {
                    // canvas.set_draw_color(Color::RGB(0, 0, 0));
                    // canvas.clear();
                    // canvas.set_draw_color(Color::RGB(255, 210, 0));
                    dbg!("Mouse {x} {y}", x, y);
                    // let surface = font
                    //     .render(&format!("{},{}", x, y))
                    //     .blended(Color::RGBA(255, 0, 0, 255))
                    //     .unwrap();
                    // let texture = texture_creator
                    //     .create_texture_from_surface(&surface)
                    //     .unwrap();

                    // let rect = Rect::new(x, y, 50, 50);
                    // canvas.draw_rect(rect).unwrap();

                    // canvas.copy(&texture, None, Some(rect)).unwrap();

                    // canvas.present();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if let Some(ref mut snake) = game.snake {
                        snake.left()
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if let Some(ref mut snake) = game.snake {
                        snake.up()
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if let Some(ref mut snake) = game.snake {
                        snake.down()
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if let Some(ref mut snake) = game.snake {
                        snake.right()
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    game.toggle_game();
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        game.render(10, 10, &mut canvas);
        canvas.present();
        game.update();
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 4));
    }
}
