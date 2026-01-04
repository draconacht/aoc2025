use colored::Colorize;
use std::{
	thread::sleep,
	time::{self, Duration, Instant},
};

#[derive(Default, Clone)]
pub enum Color {
	Primary,
	#[default]
	Secondary,
	Tertiary,
	Quaternary,
	Lin(f32),
	LinX(colored::CustomColor, colored::CustomColor, f32),
}

pub trait ToColor {
	fn to_color(&self) -> Color;
}

#[derive(Default, Clone)]
pub struct Cell {
	pub color: Color,
	pub text: char,
}

pub trait Element<X> {
	fn render(&self, inp: X) -> Vec<Vec<Cell>>;
}

pub struct Renderer {
	sleep: time::Duration,
	last_render: time::Instant,
	clrzr: fn(Color, char) -> String,
}

impl Renderer {
	pub fn new(sleep: time::Duration, clrzr: fn(Color, char) -> String) -> Self {
		Self {
			sleep,
			last_render: Instant::now(),
			clrzr,
		}
	}
}

fn interpol8(a: u8, b: u8, r: f32) -> u8 {
	a + ((b as f32 - a as f32) * r) as u8
}

pub fn default_colorizer(clr: Color, ch: char) -> String {
	let chs = ch.to_string();
	match clr {
		Color::Primary => chs.on_red().to_string(),
		Color::Secondary => chs.on_white().to_string(),
		Color::Tertiary | Color::Quaternary => chs.on_red().to_string(),
		Color::Lin(x) => chs
			.on_truecolor(60 + (140.0 * x) as u8, 70 + (140.0 * x) as u8, 140 - (12.0 * x) as u8)
			.to_string(),
		Color::LinX(a, b, x) => {
			let (br, bg, bb) = (interpol8(a.r, b.r, x), interpol8(a.g, b.g, x), interpol8(a.b, b.b, x));
			format!("\x1b[38;2;0;0;0;48;2;{br};{bg};{bb}m{chs}\x1b[0m")
		}
	}
}

impl Renderer {
	pub fn render<X>(&mut self, elem: &impl Element<X>, inp: X, caption: &str) -> Option<String> {
		// self.buffer = (elem, inp);
		if Instant::now().duration_since(self.last_render) < self.sleep {
			return None;
		}
		self.last_render = Instant::now();
		let mut out_str = "\x1B[2J\x1B[1;1H".to_string();
		let cells = elem.render(inp);
		for row in cells {
			for cell in row {
				out_str += (self.clrzr)(cell.color, cell.text).as_str();
			}
			out_str += "\n"
		}
		println!("{}", out_str);
		println!(" - {}", caption);
		Some(out_str)
		// sleep(Duration::from_millis(8));
	}
}
