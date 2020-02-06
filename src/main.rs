#[macro_use]
extern crate stdweb;

mod canvas;
mod direction;
mod snake;

use canvas::Canvas;
use direction::Direction;
use snake::Snake;

use stdweb::traits::*;
use stdweb::web::{window, document, set_timeout, event::KeyDownEvent, event::ResizeEvent, IEventTarget};

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    stdweb::initialize();

    let canvas = Rc::new(RefCell::new(Canvas::new("#canvas", 20, 20)));
    let snake = Rc::new(RefCell::new(Snake::new(20, 20)));

    snake.borrow().draw(&canvas.borrow());

    document().add_event_listener({
        let snake = snake.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "a" => snake.borrow_mut().change_direction(Direction::Left),
                "d" => snake.borrow_mut().change_direction(Direction::Right),
                "s" => snake.borrow_mut().change_direction(Direction::Down),
                "w" => snake.borrow_mut().change_direction(Direction::Up),
                _ => {}
            };
        }
    });

    window().add_event_listener({
        let canvas = canvas.clone();
        let snake = snake.clone();
        move |_: ResizeEvent| {
          canvas.borrow_mut().resize();
          snake.borrow().draw(&canvas.borrow());
        }
    });

    fn game_loop(snake: Rc<RefCell<Snake>>, canvas: Rc<RefCell<Canvas>>, time: u32) {
        set_timeout(
            move || {
                game_loop(snake.clone(), canvas.clone(), time);
                snake.borrow_mut().update();
                snake.borrow().draw(&canvas.borrow());
            },
            time,
        );
    }

    game_loop(snake, canvas, 100);

    stdweb::event_loop();
}