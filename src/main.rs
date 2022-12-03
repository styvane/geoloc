use std::env;
use std::path::Path;

use geoloc::command::Command;
use geoloc::database::Database;
use rustyline::Editor;

#[async_std::main]
async fn main() {
    if env::args().len() != 2 {
        panic!("invalid number of arguments");
    }

    let Some(path) = env::args().nth(1) else { panic!("invalid path") };
    let path = Path::new(&path);
    if !path.exists() {
        panic!("db file does not exist")
    }

    let mut db = Database::new(path);

    let mut rl = Editor::<()>::new().expect("failed to create line editor");

    println!("READY");

    while let Ok(cmd) = rl.readline("> ") {
        match Command::try_from(cmd) {
            Err(_) => println!("ERR"),
            Ok(cmd) => match db.respond(&cmd).await {
                Ok(resp) => {
                    println!("{resp}");
                    if cmd == Command::Exit {
                        break;
                    }
                }
                Err(_) => println!("ERR"),
            },
        }
    }
}
