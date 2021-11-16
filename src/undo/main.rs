#![windows_subsystem = "windows"]

use enigo::{Enigo, KeyboardControllable};

fn main() {
    let mut enigo = Enigo::new();

    // undo
    enigo.key_sequence_parse("{+CTRL}z{-CTRL}");
}
