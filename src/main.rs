use ggez::*;
use std::time::{Instant, Duration};

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 1200;
const SCREEN_BUFFER: u8 = 10;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right
}

struct Player {
    x: u32,
    y: u32,
    width: u8,
    height: u8,
    speed: u8,
    last_direction: Direction
}

impl Player {
    pub fn update(&mut self, ctx: &mut Context) {
        if self.last_direction == Direction::Right {
            self.update_pos_right();
        } else {
            self.update_pos_left();
        }
    }

    fn update_pos_right(&mut self) {
        if self.x + self.width as u32 + self.speed as u32 > SCREEN_WIDTH - SCREEN_BUFFER as u32 {
            self.x = SCREEN_WIDTH - SCREEN_BUFFER as u32 - self.width as u32
        }

        if self.x + self.width as u32 != SCREEN_WIDTH - SCREEN_BUFFER as u32 {
            self.x += self.speed as u32;
        } else {
            self.x -= self.speed as u32;
            self.last_direction = Direction::Left;
        }
    }

    fn update_pos_left(&mut self) {
        if self.x < self.speed as u32 { self.x = 0 + SCREEN_BUFFER as u32 };
        if self.x != 0 + SCREEN_BUFFER as u32 {
            self.x -= self.speed as u32;
        } else {
            self.x += self.speed as u32;
            self.last_direction = Direction::Right;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: self.x as f32,
                y: self.y as f32,
                w: self.width as f32,
                h: self.height as f32
            },
            graphics::WHITE
        ).unwrap();

        graphics::draw(
            ctx,
            &rectangle,
            graphics::DrawParam::new()
        ).unwrap();
    }
}

struct State {
    player: Player,
    gameover: bool,
    last_update: Instant
} impl State {
    pub fn new(x: u32, y: u32) -> Self {
        let player = Player {
            x: x,
            y: y,
            width: 50,
            height: 50,
            speed: 25,
            last_direction: Direction::Right
        };
        State {
            player: player,
            gameover: false,
            last_update: Instant::now()
        }
    }
}

impl ggez::event::EventHandler for State {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      self.player.update(ctx);
      Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
      println!("FPS: {}", ggez::timer::fps(ctx) as u32);
      graphics::clear(ctx, graphics::BLACK);
      self.player.draw(ctx);
      graphics::present(ctx)?;
      ggez::timer::yield_now();
      Ok(())
  }
}

pub fn main() {
    let state = &mut State::new(10, 10);
    let window_mode = conf::WindowMode {
        width: SCREEN_WIDTH as f32,
        height: SCREEN_HEIGHT as f32,
        maximized: false,
        fullscreen_type: conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };

    let window_setup = conf::WindowSetup {
        title: "Twin Stick".to_owned(),
        samples: conf::NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let c = conf::Conf::new();
    let(ref mut ctx, ref mut event_loop) = ContextBuilder::new("twin_stick", "Jon Liss")
        .conf(c)
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
