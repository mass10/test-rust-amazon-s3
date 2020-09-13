pub struct Configuration {}

#[allow(unused)]
fn between(line: &str, begin: &str, ends: &str) -> String {
	return "".to_string();
}

fn get_credentials_path() -> std::result::Result<String, Box<dyn std::error::Error>> {
	let user_profile_dir = std::env::var("USERPROFILE")?;
	let path = std::path::Path::new(&user_profile_dir).join(".aws").join("credentials");
	let path = path.to_str().unwrap().to_string();
	println!("{:?}", path);
	return Ok(path);
}

impl Configuration {
	pub fn configure(&self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// credentials の場所
		let path = get_credentials_path()?;

		// ファイルを読む
		{
			use std::io::BufRead;
			let file = std::fs::File::open(path)?;
			let r = std::io::BufReader::new(file);
			let mut current_section;
			for e in r.lines() {
				let line: String = e?;
				let line = line.trim();
				if line.starts_with("[") {
					let section = between(line, "[", "]");
					if section == "" {
						continue;
					}
					current_section = section;
					println!("[TRACE] 新しいセクション: [{}]", current_section);
					continue;
				}
				println!("[TRACE] 通常の行: [{}]", line);
			}
		}

		return Ok(());
	}
}
