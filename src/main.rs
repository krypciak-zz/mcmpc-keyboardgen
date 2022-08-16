use phf::phf_map;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const NORMAL_KEYBOARD: &str = "\n\n
        ` 1 2 3 4 5 6 7 8 9 0 - = BACKSPACE
        TAB q w e r t y u i o p [ ] ENTER
        ---- a s d f g h j k l ; ' \\\\ ENTER
        SHIFT z x c v b n m , . /  SHIFT
        --- --- SPACE_SPACE_SPACE --- --- ---- Terminate
    ";

const SHIFT_KEYBOARD: &str = "\n\n
        ~ ! @ # $ % ^ & * ( ) _ + BACKSPACE
        TAB Q W E R T Y U I O P { } ENTER
        ---- A S D F G H J K L : \\\" | ENTER
        SHIFT Z X C V B N M < > ?  SHIFT
        --- --- SPACE_SPACE_SPACE --- --- ---- Terminate
    ";

const GENERIC_KEYBOARD_KEYMAP: phf::Map<&'static str, (u8, &str)> = phf_map! {
    "TAB" => ('\t' as u8, "blue"),
    "SPACE_SPACE_SPACE" => (' ' as u8, "blue"),
    "BACKSPACE" => (32u8, "blue"),
    "SHIFT" => (0u8, "blue"),
    "ENTER" => ('\n' as u8, "blue"),
    "Terminate" => (1u8, "red"),
    "\\\\" => ('\\' as u8, "white"),
    "\\\"" => ('\"' as u8, "white"),
};

const TETRIS_KEYBOARD: &str = "\n\n
        RL RR - Lose 
        ←  ↓  → - Exit
        Drop -- Retry ---- Terminate
";

static TETRIS_KEYBOARD_KEYMAP: phf::Map<&'static str, (u8, &str)> = phf_map! {
    "RL" => ('q' as u8, "yellow"),
    "RR" => ('e' as u8, "yellow"),
    "Lose" => ('l' as u8, "white"),
    "Drop" => ('w' as u8, "blue"),
    "Retry" => ('r' as u8, "white"),
    "Exit" => ('e' as u8, "white"),
    "Terminate" => (1u8, "red"),
    "←" => ('a' as u8, "gold"),
    "→" => ('d' as u8, "gold"),
    "↓" => ('s' as u8, "gold"),
};

const CMD_BASE: &str = "#stop deleting tellraw!!
tellraw @a [";

const BASE_TEXT_0: &str = "{\"text\":\"";
const BASE_TEXT_1: &str = "\",\"bold\":true,\"color\":\"blue\"}";

const ACTION_TEXT_0: &str = "\",\"bold\":true,\"color\":\"";
const ACTION_TEXT_1: &str = "\",\"clickEvent\":{\"action\":\"run_command\",\"value\":\"";
const ACTION_TEXT_2: &str = "\"}}";

const ACTION_TO_SHIFT: &str = "/function keyboard:shift";
const ACTION_FROM_SHIFT: &str = "/function keyboard:normal";

const ACTION_TERMINATE: &str = "/function core:exit";

const FUNCTIONS_DIR: &str = "/home/krypek/Games/minecraft/instances/mcmulator/.minecraft/saves/MCMulator_v7/datapacks/MCMulator_v7/data/keyboard/functions";

static mut KEY_LIST: Vec<u8> = vec![];

fn main() {
    write_file(
        "normal",
        &gen_tellraw(NORMAL_KEYBOARD, &GENERIC_KEYBOARD_KEYMAP, false),
    );
    write_file(
        "shift",
        &gen_tellraw(SHIFT_KEYBOARD, &GENERIC_KEYBOARD_KEYMAP, true),
    );
    write_file(
        "tetris",
        &gen_tellraw(TETRIS_KEYBOARD, &TETRIS_KEYBOARD_KEYMAP, false),
    );

    unsafe {
        let iter = KEY_LIST.iter();
        for ascii_value in iter {
            let contents = format!(
                "data modify block 4 1 0 RecordItem.tag.a set value {}",
                *ascii_value as u32 * 1000
            );
            write_file(&ascii_value.to_string(), &contents);
        }
    }
}

fn write_file(name: &str, string: &str) {
    let path = format!("{}/{}.mcfunction", FUNCTIONS_DIR, name);
    let path = Path::new(&path);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(err) => panic!("Couldn't create {}: {}", display, err),
        Ok(file) => file,
    };
    // Write
    match file.write_all(string.as_bytes()) {
        Err(err) => panic!("Error writing to {}: {}", display, err),
        Ok(_) => println!("Written {}", display),
    }
}

fn gen_tellraw(
    keyboard: &str,
    keymap: &phf::Map<&'static str, (u8, &str)>,
    is_shift: bool,
) -> String {
    let keyboard = keyboard.replace("\n", " \\n");
    let mut cmd = String::from(CMD_BASE);
    let iter = keyboard.split(' ');
    for ch in iter {
        if ch.is_empty() {
            continue;
        }

        let string = get_action_text(ch, keymap, is_shift);
        cmd.push_str(string.as_str());
        cmd.push(',');
    }
    cmd = String::from(&cmd[..cmd.len() - 1]);
    cmd.push_str("]");
    cmd
}

fn get_base_text(text: &str) -> String {
    format!("{}{} {}", BASE_TEXT_0, text, BASE_TEXT_1)
}

fn get_action_text(
    text: &str,
    keymap: &phf::Map<&'static str, (u8, &str)>,
    is_shift: bool,
) -> String {
    match text {
        "----" | "---" | "--" | "-" | "\\n" => return get_base_text(text),
        _ => {}
    }

    let (val, color) = match keymap.get(text) {
        Some(tuple) => *tuple,
        None => {
            if text.len() == 1 {
                (text.chars().next().unwrap() as u8, "white")
            } else {
                panic!("Don't know what to do with: {}", text);
            }
        }
    };

    let click_action = match val {
        0 => String::from(if is_shift {
            ACTION_FROM_SHIFT
        } else {
            ACTION_TO_SHIFT
        }),
        1 => String::from(ACTION_TERMINATE),
        _ => {
            unsafe {
                KEY_LIST.push(val);
            }
            format!("/function keyboard:{}", val)
        }
    };

    format!(
        "{}{} {}{}{}{}{}",
        BASE_TEXT_0, text, ACTION_TEXT_0, color, ACTION_TEXT_1, click_action, ACTION_TEXT_2
    )
}
