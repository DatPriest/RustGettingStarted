use std::time::{SystemTime};

fn main() {
    let now = SystemTime::now();

    let mut i = 0;
    while i < 1000000000 {
        i += 1;
    }
    match now.elapsed() {
        Ok(elapsed) => {
            println!("Print var i in Million:\n{0}M", i / 1000000);
            println!("Gottcha Duration of this run = {0}", elapsed.as_millis());

        },
        Err(e) => {
            println!("An error occured: {0}", e);
        }
    }
}
