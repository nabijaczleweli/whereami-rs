extern crate whereami;

use std::path::PathBuf;


fn print_path(name: &str, path: Option<PathBuf>) {
	match path {
		Some(path) => println!("{}: {}", name, path.to_str().unwrap()),
		None => println!("Couldn't determine {} path", name),
	}
}


fn main() {
	print_path("executable", whereami::executable_path());
	print_path("module", whereami::module_path());
}
