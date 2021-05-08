use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

use crate::utils::to_string_vec;

#[derive(Debug, Deserialize)]
struct RawTOML {
	scopes: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct Config {
	pub scopes: Vec<String>,
}

pub fn read() -> Result<Config> {
	let fnames = ["resin", "commits", "conventional_commits"];
	let mut scopes: Vec<String> = to_string_vec(vec![
		"none", "lint", "deps", "release", "remove", "license", "config", "scripts",
	]);

	for i in 0..fnames.len() {
		let file_name = format!("{}{}", fnames[i], ".toml");
		let path = Path::new(&file_name);

		if path.exists() {
			let content = fs::read_to_string(path)?;
			let raw_data: RawTOML = toml::from_str(&content)?;
			scopes.extend(raw_data.scopes.unwrap_or_default());
			return Ok(Config { scopes });
		}
	}
	Ok(Config { scopes })
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;

	use crate::utils::to_string_vec;

	#[test]
	fn test_read() -> Result<()> {
		assert_eq!(
			read()?,
			Config {
				scopes: to_string_vec(vec![
					"none", "lint", "deps", "release", "remove", "license", "config", "scripts",
					"docker", "github", "actions"
				]),
			}
		);
		Ok(())
	}
}
