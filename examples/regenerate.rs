use std::{fs, path::PathBuf};

use ethos_zero::{File, Generating};
use protos::{Actualizable, Potential};

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(root.join("ethos/signal.ethos")).expect("authored Signal");
    let file = Potential::<File>::from(source)
        .actualize(())
        .expect("Signal embodiment");
    let generated = file.generate().expect("Signal generation");
    fs::write(root.join("src/generated/signal.rs"), generated).expect("generated module write");
}
