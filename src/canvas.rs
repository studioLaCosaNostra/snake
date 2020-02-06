use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

pub struct Canvas {
  pub canvas: CanvasElement,
  pub ctx: CanvasRenderingContext2d,
  scaled_width: u32,
  scaled_height: u32,
  width: u32,
  height: u32,
  max: u32,
  offset_x: u32,
  offset_y: u32,
}

fn resize_canvas_to_window_size(canvas: &CanvasElement) {
  canvas.set_width(window().inner_width() as u32);
  canvas.set_height(window().inner_height() as u32);
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

    let scaled = Canvas::scaled(&canvas, width, height);
    let max = if canvas.width() > canvas.height() {
      canvas.height()
    } else {
      canvas.width()
    };
    let offset_x = (canvas.width() - max) / 2;
    let offset_y = (canvas.height() - max) / 2;
    
    Canvas {
      canvas,
      ctx,
      scaled_width: scaled,
      scaled_height: scaled,
      width,
      height,
      max,
      offset_x,
      offset_y,
    }
  }

  pub fn scaled(canvas: &CanvasElement, width: u32, height: u32) -> u32 {
    let scaled_width = canvas.width() / width;
    let scaled_height = canvas.height() / height;
    let min = if scaled_height > scaled_width {
      scaled_width
    } else {
      scaled_height
    };
    return min;
  }

  pub fn resize(&mut self) {
    resize_canvas_to_window_size(&self.canvas);
    let scaled = Canvas::scaled(&self.canvas, self.width, self.height);
    self.scaled_height = scaled;
    self.scaled_width = scaled;
    self.max = if self.canvas.width() > self.canvas.height() {
      self.canvas.height()
    } else {
      self.canvas.width()
    };
    self.offset_x = (self.canvas.width() - self.max) / 2;
    self.offset_y = (self.canvas.height() - self.max) / 2;
  }

  pub fn draw(&self, x: u32, y: u32, color: &str) {
    assert!(x < self.width);
    assert!(y < self.height);

    self.ctx.set_fill_style_color(color);

    let x = x * self.scaled_width + self.offset_x;
    let y = y * self.scaled_height + self.offset_y;

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
      f64::from(self.offset_x),
      f64::from(self.offset_y),
      f64::from(self.max),
      f64::from(self.max),
    );

    self.ctx.set_fill_style_color("black");
    self.ctx.set_font("50px Georgia");
    let text = "W A S D";
    let width = self.ctx.measure_text(text).unwrap().get_width();
    self.ctx.fill_text(
      "W A S D",
      f64::from(self.offset_x + (self.max / 2)) - width / 2.0,
      f64::from(self.offset_y + (self.max / 2) + 25),
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
