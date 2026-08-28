# signal-ethos-zero

This repository is the generation-zero ordinary Signal contract for Ethos. It owns the
authored `ethos/signal.ethos`, its committed hand-written Rust projection, and a
validated length-prefixed rkyv frame codec. It owns no runtime, CLI, parser, string
codec, generator, storage, actors, or streams.

`examples/round-trip.datom` gives concrete positional Datomic values as contract
examples. They are deliberately specification data only: a future CLI is the sole
text-to-Signal boundary, while this crate speaks rkyv Signal only.
