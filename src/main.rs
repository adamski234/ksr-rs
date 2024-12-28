use std::{cell::RefCell, env::current_dir, error::Error, rc::Rc};

use common::variables::FileLoadError;
use rfd::MessageButtons;
use slint::{Model, ModelRc};

slint::include_modules!();

fn file_load_error_to_message(err: FileLoadError) -> String {
	match err {
		FileLoadError::FileNotFound => {
			return String::from("The requested file could not be found.")
		}
		FileLoadError::InvalidJSON => {
			return String::from("The requested file did not contain valid JSON.")
		}
		FileLoadError::PermissionDenied => {
			return String::from("Permission denied when trying to access file.")
		}
		FileLoadError::OtherError(error_data) => {
			return format!("Unknown error:\n {:#?}", error_data)
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let ui = AppWindow::new()?;

	let vars = Rc::new(RefCell::new(None));

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

	let ui_weak = ui.as_weak();
	ui.on_load_vars_pressed(move || {
		let ui = ui_weak.unwrap();
		ui.set_load_vars_open(true);
		let vars = vars.clone();
		slint::spawn_local(async move {
			let picked_file = rfd::AsyncFileDialog::new()
				.add_filter("JSON files", &["json"])
				.set_directory(current_dir().unwrap())
				.set_title("Choose JSON file with variables")
				.pick_file().await;
			if let Some(picked_file) = picked_file {
				match common::variables::parse_file(picked_file.path()) {
					Ok(data) => {
						// Should not die since buttons are locked until processing ends
						*vars.borrow_mut() = Some(data);
					}
					Err(error) => {
						rfd::AsyncMessageDialog::new()
							.set_title("Variables not loaded")
							.set_description(file_load_error_to_message(error))
							.set_buttons(MessageButtons::Ok)
							.show().await;
					}
				}
			}
			ui.set_load_vars_open(false);
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