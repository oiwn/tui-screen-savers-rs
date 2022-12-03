use crossterm::Result;
use std::io::Stdout;

const THETA_SPACING: f32 = 0.07;
const PHI_SPACING: f32 = 0.02;

#[derive(Clone)]
pub struct Donut {
    screen_width: u16,
    screen_height: u16,
    output: u16,
    zbuffer: u16,
}

impl Donut {
    pub fn new(width: u16, heigth: u16) -> Self {}
    pub fn draw(&self, stdout: &mut Stdout) {}
    pub fn update(&mut self) {}
    pub fn process_input() {}
}

pub fn run_look(stdout: &mut Stdout) -> Result<()> {}
