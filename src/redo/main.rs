#![windows_subsystem = "windows"]

use enigo::{Enigo, KeyboardControllable};

fn main() {
    let mut enigo = Enigo::new();

    // redo
    enigo.key_sequence_parse("{+CTRL}y{-CTRL}");
}
