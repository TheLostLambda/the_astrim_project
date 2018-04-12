extern crate the_astrim_project;
extern crate ggez;

use the_astrim_project::*;
use ggez::Context;
use ggez::event;
use ggez::conf;

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut Astrim::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
    let wack = the_astrim_project::cores::physics::structs::Point2 {x: 0.0, y: 0.0};
}
