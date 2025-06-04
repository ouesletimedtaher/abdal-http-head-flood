//-------------------------------------------------------------------
// Programmer       : Ebrahim Shafiei (EbraSha)
// Email            : Prof.Shafiei@Gmail.com
//-------------------------------------------------------------------

use colored::*; // اضافه کردن قابلیت رنگ‌دهی

pub fn print_ascii_art_cyberpunk() {
    let ascii_lines = r#"
 ,______________________________________       
|_________________,----------._ [____]  ""-,__  __....-----=====
               (_(||||||||||||)___________/   ""                |
                  `----------' EbraSha[ ))"-,                   |
 Abdal HTTP HEAD Flood ver 1.10        ""    `,  _,--....___    |
                                               `/           """"


Handcrafted with Passion by Ebrahim Shafiei (EbraSha)
E-Mail: Prof.Shafiei@Gmail.com
Telegram: @ProfShafiei
Github: https://github.com/ebrasha
This software is part of the Abdal arsenal, which belongs to the Abdal Security Group, led by Ebrahim Shafiei (EbraSha).
--------------------------------
"#;

    for (i, line) in ascii_lines.lines().enumerate() {
        let colored_line = match i % 6 {
            0 => line.bright_magenta(),
            1 => line.bright_cyan(),
            2 => line.bright_yellow(),
            3 => line.bright_green(),
            4 => line.bright_blue(),
            5 => line.bright_blue(),
            _ => line.bright_magenta(),
        };
        println!("{}", colored_line);
    }
}
