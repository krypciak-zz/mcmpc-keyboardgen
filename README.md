# mcmpc-keyboardgen
 Parse keyboards strings into Minecraft Tellraws  <br>  
 
Example in-game look:  
![Default keyboard](/example-looks/normal.png)  
The `SHIFT` key is a toggle:   
![Shift keyboard](/example-looks/shift.png)  
<br>
Custom keyboard layouts can be also generated.
Here's am example of a tetris keyboard:  
![Shift keyboard](/example-looks/tetris.png) 
<br>  
Here's how to the tetris layout is implemented:
```rust
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

// ....
const FUNCTIONS_DIR: &str = "~/.minecraft/saves/myworld/datapacks/mydatapack/data/keyboard/functions";
// ....

fn main() {
    // ....
    write_file(
        "tetris",
        &gen_tellraw(TETRIS_KEYBOARD, &TETRIS_KEYBOARD_KEYMAP, false),
    );
    // ....
}

```
Make sure that `FUNCTIONS_DIR` is a valid directory that exists.

<br>

# License
Licensed under GNU GPLv3 or later
