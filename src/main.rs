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
	ui.on_quantifier_child_toggled(move |tree_index, entry_index| {
		handle_checkbox_tree_child_toggle(tree_index, entry_index, ui_weak.unwrap().get_quantifiers());
	});

	let ui_weak = ui.as_weak();
	ui.on_qualifier_root_toggled(move |tree_index| {
		handle_checkbox_tree_root_toggle(tree_index, ui_weak.unwrap().get_qualifiers());
	});
	let ui_weak = ui.as_weak();
	ui.on_qualifier_child_toggled(move |tree_index, entry_index| {
		handle_checkbox_tree_child_toggle(tree_index, entry_index, ui_weak.unwrap().get_qualifiers());
	});

	let ui_weak = ui.as_weak();
	ui.on_variable_root_toggled(move |tree_index| {
		handle_checkbox_tree_root_toggle(tree_index, ui_weak.unwrap().get_variables());
	});
	let ui_weak = ui.as_weak();
	ui.on_variable_child_toggled(move |tree_index, entry_index| {
		handle_checkbox_tree_child_toggle(tree_index, entry_index, ui_weak.unwrap().get_variables());
	});

	ui.run()?;

	Ok(())
}

fn handle_checkbox_tree_root_toggle(tree_index: i32, data: ModelRc<CheckBoxTreeData>) {
	let tree_index = tree_index as usize;
	assert!(tree_index < data.row_count());
	let checkbox_tree = data.row_data(tree_index).unwrap();
	let toggle_target = checkbox_tree.root.checked;

	for child_index in 0..checkbox_tree.children.row_count() {
		let mut target = checkbox_tree.children.row_data(child_index).unwrap();
		target.checked = toggle_target;
		checkbox_tree.children.set_row_data(child_index, target);
	}
}

// FIXME: This breaks if you manually toggle the root. See https://github.com/slint-ui/slint/discussions/7217
fn handle_checkbox_tree_child_toggle(tree_index: i32, _child_index: i32, data: ModelRc<CheckBoxTreeData>) {
	let tree_index = tree_index as usize;
	assert!(tree_index < data.row_count());
	let mut checkbox_tree = data.row_data(tree_index).unwrap();
	let all_children_toggled = checkbox_tree.children.iter().all(|item| item.checked);
	checkbox_tree.root.checked = all_children_toggled;
	data.set_row_data(tree_index, checkbox_tree);
}