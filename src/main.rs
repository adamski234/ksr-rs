use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
	let ui = AppWindow::new()?;

	ui.run()?;

	Ok(())
}
