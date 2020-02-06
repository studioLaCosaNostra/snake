use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block(u32, u32);

pub struct Canvas {
  pub canvas: CanvasElement,
  pub ctx: CanvasRenderingContext2d,
  scaled_width: u32,
  scaled_height: u32,
  width: u32,
  height: u32,
  max: u32,
  offset: Block
}

fn resize_canvas_to_window_size(canvas: &CanvasElement) {
  canvas.set_width(window().inner_width() as u32);
  canvas.set_height(window().inner_height() as u32);
}

fn get_offset(canvas: &CanvasElement, max: u32) -> Block {
  let offset_x = (canvas.width() - max) / 2;
  let offset_y = (canvas.height() - max) / 2;
  return Block(offset_x, offset_y);
}

fn get_max(canvas: &CanvasElement) -> u32 {
  let max = if canvas.width() > canvas.height() {
    canvas.height()
  } else {
    canvas.width()
  };
  return max;
}

fn get_scaled(canvas: &CanvasElement, width: u32, height: u32) -> u32 {
  let scaled_width = canvas.width() / width;
  let scaled_height = canvas.height() / height;
  let min = if scaled_height > scaled_width {
    scaled_width
  } else {
    scaled_height
  };
  return min;
}

impl Canvas {
  pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
    let canvas: CanvasElement = document()
      .query_selector(attr_id)
      .unwrap()
      .unwrap()
      .try_into()
      .unwrap();

    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
    resize_canvas_to_window_size(&canvas);

    let scaled = get_scaled(&canvas, width, height);
    let max = get_max(&canvas);
    let offset = get_offset(&canvas, max);

    Canvas {
      canvas,
      ctx,
      scaled_width: scaled,
      scaled_height: scaled,
      width,
      height,
      max,
      offset
    }
  }

  pub fn resize(&mut self) {
    resize_canvas_to_window_size(&self.canvas);
    let scaled = get_scaled(&self.canvas, self.width, self.height);
    self.scaled_height = scaled;
    self.scaled_width = scaled;
    self.max = get_max(&self.canvas);
    self.offset = get_offset(&self.canvas, self.max);
  }

  pub fn draw(&self, x: u32, y: u32, color: &str) {
    assert!(x < self.width);
    assert!(y < self.height);

    self.ctx.set_fill_style_color(color);

    let x = x * self.scaled_width + self.offset.0;
    let y = y * self.scaled_height + self.offset.1;

    self.ctx.fill_rect(
      f64::from(x),
      f64::from(y),
      f64::from(self.scaled_width),
      f64::from(self.scaled_height),
    );
  }

  pub fn draw_board(&self) {
    self.ctx.set_fill_style_color("black");
    self.ctx.fill_rect(
      f64::from(0),
      f64::from(0),
      f64::from(self.canvas.width()),
      f64::from(self.canvas.height()),
    );
    self.ctx.set_fill_style_color("blue");
    self.ctx.fill_rect(
      f64::from(self.offset.0),
      f64::from(self.offset.1),
      f64::from(self.max),
      f64::from(self.max),
    );

    self.ctx.set_fill_style_color("black");
    self.ctx.set_font("50px Georgia");
    let text = "W A S D";
    let width = self.ctx.measure_text(text).unwrap().get_width();
    self.ctx.fill_text(
      "W A S D",
      f64::from(self.offset.0 + (self.max / 2)) - width / 2.0,
      f64::from(self.offset.1 + (self.max / 2) + 25),
      Some(width)
    );
  }

  pub fn clear_all(&self) {
    self.ctx.set_fill_style_color("white");
    self.ctx.fill_rect(
      f64::from(0),
      f64::from(0),
      f64::from(self.canvas.width()),
      f64::from(self.canvas.height()),
    );
  }
}
