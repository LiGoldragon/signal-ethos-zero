# signal-ethos-zero

Generation-zero ordinary Signal vocabulary for Ethos, version 2.0.0: the
wire of the Ethos Nexus that follows ethos-zero, which nothing serves yet.

`ethos/library.ethos` (the names shared with meta-signal-ethos-zero) and
`ethos/signal.ethos` are the authored sources, written in ethos-zero's own
vertical print; `src/generated/` is their ethos-zero 16.0.0 projection, held
byte-identical, and the sources held in that print, by `build.rs`. Regenerate
with `ethos-zero 'Generate.{ /abs/ethos/<file>.ethos /abs/src/generated }'`.

The contract rides signal 8.0.0's exchange layer: a connection is greeted once
with the digest of the two sources (`signal::Contracted for Query`), each
query travels as `Dispatch::Open` on an exchange the peer names, and each
response comes back as `Delivery::Answer` on that exchange until
`Delivery::End`. Datom text is behind the `datom` feature, for the CLI only.
