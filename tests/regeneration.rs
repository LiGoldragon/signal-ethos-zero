use std::fs;

use ethos_zero::{File, Generating};
use protos::{Actualizable, Potential};

#[test]
fn generated_signal_is_current_ethos_projection() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(root.join("ethos/signal.ethos")).expect("authored Signal");
    let file = Potential::<File>::from(source)
        .actualize(())
        .expect("Signal embodiment");
    let generated = file.generate().expect("Signal generation");
    assert_eq!(
        fs::read_to_string(root.join("src/generated/signal.rs"))
            .expect("committed generated module"),
        generated,
        "src/generated/signal.rs must be regenerated from ethos/signal.ethos"
    );
}
