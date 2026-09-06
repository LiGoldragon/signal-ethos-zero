# signal-ethos-zero

This repository is the generation-zero ordinary Signal contract for Ethos. It owns the
authored `ethos/signal.ethos`, its committed hand-written Rust projection, and a
validated length-prefixed rkyv exchange frame codec. Its globally allocated binding
is ContractId 7 at revision 4; 1/2 remain the historical Orchestrate allocations,
3/4 belong to Criome, and 5/6 belong to Lojix. Every frame validates the
EthosZero binding and route before its body is accepted. It owns no runtime, CLI, parser, string
codec, generator, storage, actors, or streams.

`examples/round-trip.datom` gives concrete positional Datomic values as contract
examples. They are deliberately specification data only: a future CLI is the sole
text-to-Signal boundary, while this crate speaks rkyv Signal only.
