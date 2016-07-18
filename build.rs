extern crate gcc;


fn main() {
    gcc::Config::new()
        .file("ext/whereami/src/whereami.c")
        .include("ext/whereami/src")
        .compile("libwhereami.a");
}
