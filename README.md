# signal-ethos-zero

Generation-zero ordinary Signal vocabulary for Ethos, version 1.0.0: the
wire of the Ethos Nexus that follows ethos-zero, which nothing serves yet.

`ethos/signal.ethos` is the authored source; `src/generated/signal.rs` is its
ethos-zero 15.0.0 projection, held byte-identical by `build.rs`. Regenerate it
with `ethos-zero 'Generate.{ /abs/ethos/signal.ethos /abs/src/generated }'`.

The contract rides signal 7.0.0's exchange layer: a connection is greeted once
with the digest of `ethos/signal.ethos` (`signal::Contracted for Query`), each
query travels as `Dispatch::Open` on an exchange the peer names, and each
response comes back as `Delivery::Answer` on that exchange until
`Delivery::End`. Datom text is behind the `datom` feature, for the CLI only.
