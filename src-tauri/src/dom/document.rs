use std::fmt::{Display, Formatter};

// lets not talk about it, unless you know a way around doing this.
#[derive(Copy, Clone)]
pub struct Element<'a> {
	tag: &'a str,
	classes: &'a [&'a str],
	id: Option<&'a str>,
	content: &'a str,
	attributes: &'a [(&'a str, &'a str)],
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
		let attributes = {
			if self.attributes.is_empty() {
				"".to_owned()
			} else {
				let mut attrs = "".to_owned();
				self.attributes.iter().map(|x| attrs.push_str(&format!("{}=\"{}\"", x.0, x.1)));
				attrs
			}
		};

		write!(f, "<{0}{id}{classes}{attributes}>{1}</{0}>", self.tag, self.content.trim())
	}
}

impl <'a> Element<'a> {
	pub fn build(tag: &'a str) -> Self {
		Self {
			tag,
			classes: &[],
			id: None,
			content: "",
			attributes: &[]
		}
	}
	pub fn id(&self, id: &'a str) -> Self {
		Self {
			tag: self.tag,
			classes: self.classes,
			id: Some(id),
			content: self.content,
			attributes: self.attributes,
		}
	}
	pub fn classes(&self, classes: &'a [&'a str]) -> Self {
		Self {
			tag: self.tag,
			classes,
			id: self.id,
			content: self.content,
			attributes: self.attributes,
		}
	}
	pub fn content(&self, content: &'a str) -> Self {
		Self {
			tag: self.tag,
			classes: self.classes,
			id: self.id,
			content,
			attributes: self.attributes,
		}
	}
	pub fn attributes(&self, attributes: &'a [(&'a str, &'a str)]) -> Self {
		Self {
			tag: self.tag,
			classes: self.classes,
			id: self.id,
			content: self.content,
			attributes,
		}
	}
	pub fn new(tag: &'a str, id: Option<&'a str>, classes: &'a [&'a str], content: &'a str, attributes: &'a [(&'a str, &'a str)]) -> Self {
		Self {
			tag,
			classes,
			id,
			content,
			attributes,
		}
	}
	pub fn append_to_id(&self, target: &'a str, window: &'a tauri::Window<>) -> Result<(), ()> {
		let to_eval = format!("document.getElementById(\"{target}\").insertAdjacentHTML('beforeend', \'{self}\');");
		window.eval(&to_eval).unwrap();
		Ok(())
	}
}