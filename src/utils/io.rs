pub fn write_with_newline<W: std::io::Write>(writer: &mut W, message: &str) {
    writeln!(writer, "\r{}\r\n", message).unwrap(); // Ensure proper formatting with carriage return and newline
}