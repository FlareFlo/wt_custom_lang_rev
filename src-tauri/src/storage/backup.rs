use std::fs;

use chrono::NaiveDateTime;
use fs_extra::dir::CopyOptions;
use serde::{Deserialize, Serialize};

use crate::custom_lang::custom_lang::CustomLang;

use lazy_static::lazy_static;
use duckstore::{DirType, PathConfig, ResolvedPaths};

use crate::{CONFIG_NAME};

pub const BACKUP_CONFIG: PathConfig = PathConfig {
	project_prefix: CONFIG_NAME,
	sub_folder: "backup",
	file_name: "placeholder",
	dir_type: &DirType::Data,
};

lazy_static! {
 pub static ref BACKUP_PATH: ResolvedPaths<'static> = {
		match BACKUP_CONFIG.resolve() {
			Ok(x) => x,
			// Unsafe unwrap as this error cannot be recovered at runtime
			Err(err) => panic!("{err}"),
		}
    };
	pub static ref BACKUP_ROOT: String = format!("{}/{}/{}", &BACKUP_PATH.base_path, &BACKUP_PATH.config.project_prefix, &BACKUP_PATH.config.sub_folder);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromptForBackup {
	pub active: bool,
	pub backup_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BackupEntry {
	pub name: String,
	pub dest: String,
	pub date: i64,
}

pub const BACKUP_ENTRY_STORAGE: fn() -> String = || {
	format!("{}/{}/{}/backups.json", &BACKUP_PATH.base_path, &BACKUP_PATH.config.project_prefix, &BACKUP_PATH.config.sub_folder)
};

pub const WRITE_BACKUP: fn(&mut CustomLang, &Vec<BackupEntry>) = |custom_lang, backups| {
	match serde_json::to_string(&backups) {
		Ok(bin) => {
			match fs::write(&BACKUP_ENTRY_STORAGE(), bin) {
				Ok(_) => {}
				Err(error) => {
					custom_lang.error(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
				}
			}
		}
		Err(error) => {
			custom_lang.error(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
		}
	}
};

pub const READ_BACKUP: fn(&mut CustomLang) -> Option<Vec<BackupEntry>> = |custom_lang| {
	return match fs::read_to_string(&BACKUP_ENTRY_STORAGE()) {
		Ok(bin) => {
			match serde_json::from_str(&bin) {
				Ok(serialized) => {
					Some(serialized)
				}
				Err(error) => {
					custom_lang.error(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
					None
				}
			}
		}
		Err(error) => {
			custom_lang.error(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
			None
		}
	};
};

const COPY_OPTIONS: CopyOptions = CopyOptions {
	overwrite: true,
	skip_exist: false,
	buffer_size: 64000,
	copy_inside: false,
	content_only: true,
	depth: 0,
};

impl CustomLang {
	fn create_backup(&mut self, wt_folder: &str) {
		let name = &self.prompt_for_backup.backup_name;
		if let Ok(bin) = fs::read_to_string(&BACKUP_ENTRY_STORAGE()) {
			let opt_bin: Result<Vec<BackupEntry>, serde_json::Error> = serde_json::from_str(&bin);
			match opt_bin {
				Ok(mut backups) => {
					let time = chrono::Local::now();
					let path = format!("{}/{}/{}/backup_{}", &BACKUP_PATH.base_path, &BACKUP_PATH.config.project_prefix, &BACKUP_PATH.config.sub_folder, &time.timestamp().to_string());

					match fs::create_dir_all(&path) {
						Ok(_) => {
							if let Err(err) = fs_extra::dir::copy(wt_folder, &path, &COPY_OPTIONS) {
								self.error(format!("{:?} {}:{} {}", err, line!(), column!(), file!()));
								return;
							} else {
								backups.push(BackupEntry {
									name: name.to_owned(),
									dest: path.clone(),
									date: time.timestamp(),
								});
								WRITE_BACKUP(self, &backups);
							}
						}
						Err(error) => {
							self.error(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
							return;
						}
					}
				}
				Err(error) => {
					self.error(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
					return;
				}
			}
		}
	}

	pub fn prompt_for_backup(&mut self) {
		if let Some(raw_path) = &self.config.wt_path.as_ref() {
			let wt_path = format!("{}/lang", raw_path);

			unimplemented!("Add a new window for backup creation here");
			// Window::new("Manage backups").show(ctx, |ui| {
			// 	ui.horizontal(|ui| {
			// 		ui.add(TextEdit::singleline(&mut self.prompt_for_backup.backup_name));
			// 		if ui.add(Button::new("Create backup")).clicked() {
			// 			self.create_backup(&wt_path);
			// 		}
			// 	});
			//
			// 	ui.add_space(15.0);
			//
			//
			// 	if let Some(mut backups) = READ_BACKUP(self) {
			// 		for (i, backup) in backups.clone().iter().enumerate() {
			// 			ui.horizontal(|ui| {
			// 				let time = NaiveDateTime::from_timestamp(backup.date, 0).to_string();
			// 				ui.add(Label::new(format!("Name: {} Created: {:?}", &backup.name, time)));
			// 				if ui.add(Button::new("Load")).clicked() {
			// 					match fs_extra::dir::copy(&backup.dest, &wt_path, &COPY_OPTIONS) {
			// 						Ok(_) => {}
			// 						Err(error) => {
			// 							self.prompt_error.err_value = Some(format!("{:?} {}:{} {}", error, line!(), column!(), file!()));
			// 							return;
			// 						}
			// 					}
			// 				}
			//
			// 				if ui.add(Button::new(RichText::new("Delete backup").color(Color32::from_rgb(255, 0, 0)))).clicked() {
			// 					backups.remove(i);
			// 				}
			// 			});
			// 		}
			//
			// 		ui.add_space(15.0);
			//
			// 		if ui.add(Button::new("Close")).clicked() {
			// 			self.prompt_for_backup.active = false;
			// 		}
			//
			// 		WRITE_BACKUP(self, &backups);
			// 	}
			// 	// No else as error handling will be handled in the next iteration
			// });
		} else {
			self.error("No WT path is set, but at this point in time it should be".to_owned());
			return;
		}
	}
}