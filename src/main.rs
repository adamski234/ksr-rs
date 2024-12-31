use std::{cell::RefCell, env::current_dir, error::Error, ops::Deref, rc::Rc};

use common::{data, variables::{LinguisticVariable, VariableFile}};
mod error_dialog;
use slint::{Model, ModelRc, VecModel};

slint::include_modules!();

fn variables_to_checkboxes(vars: &[LinguisticVariable]) -> ModelRc<CheckBoxTreeData> {
	return ModelRc::new(vars.iter().map(|item| {
		return CheckBoxTreeData { 
			root: CheckBoxListElementData { checked: false, text: item.name.clone().into() },
			children: ModelRc::new(item.labels.iter().map(|label| {
				return CheckBoxListElementData { checked: false, text: label.name.clone().into()};
			}).collect::<VecModel<_>>()),
		};
	}).collect::<VecModel<_>>());
}

fn main() -> Result<(), Box<dyn Error>> {
	let ui = AppWindow::new()?;

	let vars = Rc::new(RefCell::new(None));
	let data = Rc::new(RefCell::new(None));

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

	let vars_clone = vars.clone();
	let ui_weak = ui.as_weak();
	ui.on_load_vars_pressed(move || {
		let ui = ui_weak.unwrap();
		ui.set_load_vars_open(true);
		let vars = vars_clone.clone();
		slint::spawn_local(async move {
			let picked_file = rfd::AsyncFileDialog::new()
				.add_filter("JSON files", &["json"])
				.set_directory(current_dir().unwrap())
				.set_title("Choose JSON file with variables")
				.pick_file().await;
			if let Some(picked_file) = picked_file {
				match VariableFile::parse_file(picked_file.path()) {
					Ok(data) => {
						// Should not die since buttons are locked until processing ends
						ui.set_quantifiers(variables_to_checkboxes(&data.quantifiers));
						ui.set_qualifiers(variables_to_checkboxes(&data.linguistic_variables));
						ui.set_variables(variables_to_checkboxes(&data.linguistic_variables));
						*vars.borrow_mut() = Some(data);
						ui.set_variables_loaded(true);
					}
					Err(error) => {
                        error_dialog::show_error_dialog("Variables not loaded", error).await;
					}
				}
			}
			ui.set_load_vars_open(false);
		}).unwrap();
	});

	let vars_clone = vars.clone();
	let ui_weak = ui.as_weak();
	ui.on_save_vars_pressed(move || {
		let ui = ui_weak.unwrap();
		ui.set_save_vars_open(true);
		let vars = vars_clone.clone();
		slint::spawn_local(async move {
			if let Some(data) = &vars.borrow().deref() {
				let picked_file = rfd::AsyncFileDialog::new()
					.add_filter("JSON files", &["json"])
					.set_directory(current_dir().unwrap())
					.set_title("Choose JSON file with variables")
					.save_file().await;
				if let Some(picked_file) = picked_file {
					if let Err(error) = data.save_file(picked_file.path()) {
                        error_dialog::show_error_dialog("Variables not saved", error).await;
					}
				}
			}
			ui.set_save_vars_open(false);
		}).unwrap();
	});

	let data_clone = data.clone();
	let ui_weak = ui.as_weak();
	ui.on_load_data_pressed(move || {
		let ui = ui_weak.unwrap();
		ui.set_load_data_open(true);
		let data = data_clone.clone();
		slint::spawn_local(async move {
			let picked_file = rfd::AsyncFileDialog::new()
				.add_filter("SQLite databases", &["db"])
				.set_directory(current_dir().unwrap())
				.set_title("Choose the database")
				.pick_file().await;
			if let Some(picked_file) = picked_file {
				let loaded_data = data::Sample::load_from_db(picked_file.path()).await;
				ui.set_sample_count(loaded_data.len() as i32);
				*data.borrow_mut() = Some(loaded_data);
				ui.set_data_loaded(true);
			}
			ui.set_load_data_open(false);
		}).unwrap();
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