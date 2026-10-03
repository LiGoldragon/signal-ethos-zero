//! This contract inside signal's exchange envelope, exercised as a wire.
//!
//! Every value is framed, read back off a byte stream and attributed to its
//! exchange. The one thing taken from outside the crate is the digest oracle,
//! computed by the published FNV-1a algorithm over `ethos/library.ethos` then
//! `ethos/signal.ethos` in Python rather than through the path under test.

use std::io::Cursor;

use signal::{
    Answer, ByteViewable, ContractDigest, Contracted, Delivery, Dispatch, Ending, ExchangeId,
    ExchangeLedger, ExchangeMinting, Exchanged, FrameCapacity, FrameReading, FrameWriting, Greeted,
    Handshake, HandshakeReceipt, HandshakeRejection, Opening, Restorable, Signal, Signalizable,
};
use signal_ethos_zero::{ETHOS, FileLocation, Generation, GenerationRefusal, Query, Response};

/// FNV-1a over exactly the bytes of `ethos/library.ethos` followed by those of
/// `ethos/signal.ethos`, as a signed 64-bit integer.
const ORDINARY_DIGEST: ContractDigest = -1_927_924_637_596_380_248;

/// A peer built from another source: the same envelope, another contract.
struct OtherContract;

impl Contracted for OtherContract {
    const CONTRACT_SOURCE: &'static str = "Signal\n[]\n[ Ping.Integer ]\n[ Pong ]\n[]\n";
}

/// Put a value on a byte stream the way a socket carries it, and read it back.
trait CrossesTheWire: Sized {
    fn across(&self) -> Self;
}

impl<T> CrossesTheWire for T
where
    T: Signalizable,
    Signal<T>: Restorable<T>,
{
    fn across(&self) -> Self {
        let capacity = FrameCapacity::default();
        let mut wire = Vec::new();
        wire.write_frame(&self.signalize().expect("signalize"), capacity)
            .expect("write the frame");
        let body = Cursor::new(wire)
            .read_frame(capacity)
            .expect("read the frame");
        Signal::<T>::from(body.bytes().to_vec())
            .restore()
            .expect("restore the value")
    }
}

trait Located {
    fn located(path: &str) -> Self;
}

impl Located for FileLocation {
    fn located(path: &str) -> Self {
        Self {
            source_name: "workspace".to_owned(),
            relative_path: path.to_owned(),
        }
    }
}

/// A greeted ledger of the querying side, where exchange identifiers come
/// from.
trait Opens {
    fn greeted() -> Self;
}

impl Opens for ExchangeLedger {
    fn greeted() -> Self {
        let mut ledger = Self::default();
        ledger.greet().expect("the first greeting");
        ledger
    }
}

#[test]
fn the_contract_is_identified_by_the_digest_of_its_own_authored_source() {
    assert_eq!(<Query as Contracted>::contract_digest(), ORDINARY_DIGEST);
    assert_eq!(
        <Query as Contracted>::greeting(),
        Handshake {
            contract_digest: ORDINARY_DIGEST
        }
    );
    assert_eq!(<Query as Contracted>::CONTRACT_SOURCE, ETHOS);
}

#[test]
fn a_peer_built_from_another_source_is_refused_rather_than_negotiated_with() {
    assert_eq!(
        <Query as Contracted>::receipt(&OtherContract::greeting()),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(ORDINARY_DIGEST))
    );
    assert_eq!(
        <Query as Contracted>::receipt(&<Query as Contracted>::greeting()),
        HandshakeReceipt::Greeted(ORDINARY_DIGEST)
    );
}

#[test]
fn the_greeting_and_a_generate_cross_the_wire_as_dispatches() {
    let greeting: Dispatch<Query> = Dispatch::Greet(<Query as Contracted>::greeting());
    assert_eq!(greeting.across(), greeting);

    let mut ledger = ExchangeLedger::greeted();
    let exchange = ledger.open().expect("open an exchange");
    let opening = Dispatch::Open(Opening {
        exchange,
        query: Query::Generate(FileLocation::located("ethos/signal.ethos")),
    });
    let received = opening.across();
    assert_eq!(received, opening);
    let Dispatch::Open(received) = received else {
        panic!("a query crosses the wire inside the exchange it opens");
    };
    assert_eq!(received.exchange(), exchange);
    assert!(!received.is_connection_wide());
}

#[test]
fn a_refused_generation_is_an_answer_and_then_an_ending() {
    let exchange: ExchangeId = 1;
    let refusal: Delivery<Response> = Delivery::Answer(Answer {
        exchange,
        response: Response::GenerationRejected(GenerationRefusal::FileAbsent(
            FileLocation::located("ethos/missing.ethos"),
        )),
    });
    assert_eq!(refusal.across(), refusal);
    let Delivery::End(ending) = Delivery::<Response>::End(Ending::completed(exchange)).across()
    else {
        panic!("a one-answer exchange ends after its answer");
    };
    assert_eq!(ending.exchange(), exchange);
}

#[test]
fn a_subscription_and_a_generate_are_told_apart_by_exchange_alone() {
    let capacity = FrameCapacity::default();
    let mut ledger = ExchangeLedger::greeted();
    let watching = ledger.open().expect("open the subscription");
    let generating = ledger.open().expect("open the Generate exchange");
    let request = FileLocation::located("ethos/signal.ethos");
    let generation = Generation {
        file_location: FileLocation::located("ethos/signal.ethos"),
        artifact_path: "src/generated/signal.rs".to_owned(),
    };

    let opened = vec![
        Dispatch::Open(Opening {
            exchange: watching,
            query: Query::Subscribe(FileLocation::located("ethos/signal.ethos")),
        }),
        Dispatch::Open(Opening {
            exchange: generating,
            query: Query::Generate(request.clone()),
        }),
    ];
    for dispatch in &opened {
        assert_eq!(&dispatch.across(), dispatch);
    }

    // The order the answering side writes them in: the subscription sees the
    // generation start, the Generate is answered and ended, and the
    // subscription then sees it complete.
    let written: Vec<Delivery<Response>> = vec![
        Delivery::Greeted(HandshakeReceipt::Greeted(ORDINARY_DIGEST)),
        Delivery::Answer(Answer {
            exchange: watching,
            response: Response::GenerationStarted(request),
        }),
        Delivery::Answer(Answer {
            exchange: generating,
            response: Response::Generated(generation.clone()),
        }),
        Delivery::End(Ending::completed(generating)),
        Delivery::Answer(Answer {
            exchange: watching,
            response: Response::GenerationCompleted(generation.clone()),
        }),
    ];
    let mut wire = Vec::new();
    for delivery in &written {
        wire.write_frame(&delivery.signalize().expect("signalize"), capacity)
            .expect("write one frame");
    }
    let mut stream = Cursor::new(wire);
    let read = written
        .iter()
        .map(|_| {
            let body = stream.read_frame(capacity).expect("read one frame");
            Signal::<Delivery<Response>>::from(body.bytes().to_vec())
                .restore()
                .expect("restore one delivery")
        })
        .collect::<Vec<_>>();
    assert_eq!(read, written);

    let watched = read
        .iter()
        .filter_map(|delivery| match delivery {
            Delivery::Answer(answer) if answer.exchange() == watching => Some(&answer.response),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(watched.len(), 2);
    assert!(matches!(watched[0], Response::GenerationStarted(_)));
    assert_eq!(watched[1], &Response::GenerationCompleted(generation));

    let abandon: Dispatch<Query> = Dispatch::Abandon(watching);
    assert_eq!(abandon.across(), Dispatch::Abandon(watching));
}
