use web_sys::Document;
use web_sys::Window;
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

	let prim = PrimitiveEntry {
		file: "".to_string(),
		id: None,
		original_english: "old".to_string(),
		new_english: "new".to_string()
	};

	prim_array = vec![prim];

	let mut inner = "".to_owned();
	for (i, primitive_entry) in prim_array.iter().enumerate() {
		let left = Element::new("td", None, &[], &primitive_entry.original_english).unwrap();
		let right = Element::new("td", None, &[], &primitive_entry.new_english).unwrap();
		let content = format!("{left}{right}");

		let tr = Element::new("tr", None, &[], &content).unwrap();
		inner += &tr.to_string();
		println!("sus");
	}

	let table = Element::new("table", None, &[], &inner).unwrap();
	println!("{}", &table);
	table.append_to_id("body", &window).unwrap();
}