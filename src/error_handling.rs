pub fn error(line: usize, index: usize, msg: &str) {
    println!(
        "\x1b[1;31merror: \x1b[0m{} on line [{}:{}]",
        msg, line, index
    );
}

pub fn warning(line: usize, index: usize, msg: &str) {
    println!(
        "\x1b[1;33mwarning: \x1b[0m{} on line [{}:{}]",
        msg, line, index
    );
}
