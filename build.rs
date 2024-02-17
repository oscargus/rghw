fn main() {
    cc::Build::new()
        .files(&["libghw/libghw.c"])
        .compile("libghw");
}
