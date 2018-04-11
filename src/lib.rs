extern crate ggez;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::event;

pub struct Astrim {
    pos_x: f32,
}

impl Astrim {
    pub fn new(_ctx: &mut Context) -> GameResult<Astrim> {
        let s = Astrim { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for Astrim {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx, DrawMode::Fill, Point2::new(self.pos_x, 380.0), 100.0, 0.1)?;
        graphics::present(ctx);
        Ok(())
    }
}
