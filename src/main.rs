use std::error::Error;

use slint::{Model, ModelRc};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
	let ui = AppWindow::new()?;

	let ui_weak = ui.as_weak();
	ui.on_quantifier_root_toggled(move |tree_index| {
		handle_checkbox_tree_root_toggle(tree_index, ui_weak.unwrap().get_quantifiers());
	});

	let ui_weak = ui.as_weak();
	ui.on_qualifier_root_toggled(move |tree_index| {
		handle_checkbox_tree_root_toggle(tree_index, ui_weak.unwrap().get_qualifiers());
	});

	let ui_weak = ui.as_weak();
	ui.on_variable_root_toggled(move |tree_index| {
		handle_checkbox_tree_root_toggle(tree_index, ui_weak.unwrap().get_variables());
	});

	ui.run()?;

	Ok(())
}

fn handle_checkbox_tree_root_toggle(tree_index: i32, data: ModelRc<CheckBoxTreeData>) {
	let tree_index = tree_index as usize;
	assert!(tree_index < data.row_count());
	let quantifier_tree = data.row_data(tree_index).unwrap();
	let toggle_target = quantifier_tree.root.checked;

	for index in 0..quantifier_tree.children.row_count() {
		let mut target = quantifier_tree.children.row_data(index).unwrap();
		target.checked = toggle_target;
		quantifier_tree.children.set_row_data(index, target);
	}
}