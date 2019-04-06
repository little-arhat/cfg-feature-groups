mod cfg;

#[cfg(log = "opt1")]
fn log() {
    println!("opt1")
}

#[cfg(log = "opt2")]
fn log() {
    println!("opt2")
}

fn main() {
    log();
}
