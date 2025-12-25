use colored::Colorize;
use std::{thread::sleep, time};

#[derive(Default, Clone)]
pub enum Color {
	Primary,
	#[default]
	Secondary,
	Tertiary,
	Quaternary,
	Lin(f32),
}

#[derive(Default, Clone)]
pub(super) struct Cell {
	pub color: Color,
	pub text: char,
}

pub(super) trait Element<X> {
	fn render(&self, inp: X) -> Vec<Vec<Cell>>;
}

pub struct Renderer {
	pub sleep: time::Duration,
	pub clrzr: fn(Color, char) -> String,
}

pub fn default_colorizer(clr: Color, ch: char) -> String {
	let chs = ch.to_string();
	match clr {
		Color::Primary => chs.on_green(),
		Color::Secondary => chs.on_white(),
		Color::Tertiary | Color::Quaternary | Color::Lin(_) => chs.on_red(),
	}
	.to_string()
}

impl Renderer {
	pub fn render<X>(&self, elem: &impl Element<X>, inp: X, caption: &str) {
		// print!("\x1B[2J\x1B[1;1H");
		let cells = elem.render(inp);
		for row in cells {
			for cell in row {
				print!("{}", (self.clrzr)(cell.color, cell.text));
			}
		}
		println!(" - {}", caption);
		sleep(self.sleep);
	}
}
