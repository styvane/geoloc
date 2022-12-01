use std::env;

use geoloc::command::Command;
use geoloc::database::Database;
use rustyline::Editor;

fn main() {
    if env::args().len() != 2 {
        panic!("invalid number of arguments");
    }

    let Some(path) = env::args().nth(1) else { panic!("invalid path") };
    let mut db = Database::new(path).expect("failed to create db");
    let mut rl = Editor::<()>::new().expect("failed to create line editor");
    println!("READY");
    while let Ok(cmd) = rl.readline("> ") {
        let cmd = Command::try_from(cmd).expect("failed to create command");
        let response = db.respond(&cmd).expect("failed to respond");
        println!("{response}");
        if cmd == Command::Exit {
            break;
        }
    }
}
