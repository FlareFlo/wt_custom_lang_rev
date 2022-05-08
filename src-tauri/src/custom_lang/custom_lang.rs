use web_sys::{Document};
use crate::cache::cache::Cache;
use crate::config::Configuration;
use crate::dom::document::Element;
use crate::lang_manipulation::primitive_lang::PrimitiveEntry;
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
	let mut prim_array = READ_PRIMITIVE();

	let mut inner = "".to_owned();
	for (i, primitive_entry) in prim_array.iter().enumerate() {
		let old = Element::build("td").content(&primitive_entry.original_english);

		let new =  Element::build("td").content(&primitive_entry.new_english);


		let txt = format!("{i}_btn");
		let delete =  Element::build("button").classes(&["delete_btn"]).id(&txt).content("Delete").to_string();
		let delete_td = Element::build("td").content(&delete);

		let content = format!("{old}{new}{delete_td}");

		let id = format!("{i}_entries");
		let tr = Element::build("tr").content(&content).id(&id);
		inner += &tr.to_string();
	}

	let table = Element::build("table").content(&inner);
	table.append_to_id("body", &window).unwrap();
}