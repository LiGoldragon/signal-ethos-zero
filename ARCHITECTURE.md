# signal-ethos-zero

This repository is the generation-zero ordinary Signal contract for Ethos. It
owns the authored `ethos/library.ethos` and `ethos/signal.ethos` and the
committed Rust that ethos-zero generates from them, and nothing else: no runtime, CLI, parser, generator,
storage or framing.

Framing, the greeting and the exchange envelope are signal's. The contract's
wire identity is the digest of its authored sources, so a peer built from any
other source is refused at the greeting rather than negotiated with; there is
no allocated contract number and no revision. Every generated type archives
with rkyv; its datom derives sit behind the `datom` feature, which a CLI
enables and a Nexus does not.

`examples/round-trip.datom` holds the datom text of concrete values; the
`datom` feature's test reads every line and reads it back equal. Production
Rust carries no free function and no inherent method, held by the
`no-free-functions` and `no-inherent-methods` flake checks.

The Library holds the names the ordinary and meta contracts share; the
Signal, and meta-signal-ethos-zero's Signal, import them as
`signal_ethos_zero:[ … ]`. A type used once is declared inline where it is
used; a type used twice is declared once and named; a variant carries its
payload itself, with no holder type.
