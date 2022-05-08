use std::fmt::{Display, Formatter};

// lets not talk about it, unless you know a way around doing this.
#[derive(Copy, Clone)]
pub struct Element<'a> {
	pub tag: &'a str,
	pub classes: &'a [&'a str],
	pub id: Option<&'a str>,
	pub content: &'a str,
}

impl <'a> Display for Element<'a>  {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let id = {
			if let Some(id) = self.id {
				format!(" id=\"{id}\"")
			} else {
				"".to_owned()
			}
		};
		let classes = {
			if self.classes.len() == 0 {
				"".to_owned()
			} else {
				" class= \"".to_owned() + &self.classes.join(" ") + &"\""
			}
		};

		write!(f, "<{0}{id}{classes}>{1}</{0}>", self.tag, self.content.trim())
	}
}

impl <'a> Element<'a> {
	pub fn new(tag: &'a str, id: Option<&'a str>, classes: &'a [&'a str], content: &'a str) -> Result<Self, ()> {
		Ok(Self {
			tag,
			classes,
			id,
			content,
		})
	}
	pub fn append_to_id(&self, target: &'a str, window: &'a tauri::Window<>) -> Result<(), ()> {
		let to_eval = format!("document.getElementById(\"{target}\").insertAdjacentHTML('beforeend', \'{self}\');");
		window.eval(&to_eval).unwrap();
		Ok(())
	}
}