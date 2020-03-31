/* ** Input Stream ** */

use std::io;

pub fn read_data () -> f64 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<f64>() {
        Ok(i) => return i,
        Err(..) => return 0.0,
    };
}
