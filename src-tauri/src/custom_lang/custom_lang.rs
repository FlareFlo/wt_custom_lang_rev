use crate::cache::cache::Cache;
use crate::config::Configuration;
use crate::storage::backup::PromptForBackup;
use crate::storage::entries::READ_PRIMITIVE;

pub struct CustomLang {
	pub config: Configuration,
	pub status_menu: bool,
	pub prompt_for_backup: PromptForBackup,
	pub cache: Cache,
}

#[tauri::command]
pub fn list_all(window: tauri::Window) {
	let prim_array = READ_PRIMITIVE();

	for (i, primitive_entry) in prim_array.iter().enumerate() {
		// Set up entry list here
	}


	println!("I was invoked from JS!");
}