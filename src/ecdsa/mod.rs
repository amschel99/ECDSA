use num_bigint::{BigInt, BigUint};

pub struct Point {
    pub x: BigUint,
    pub y: BigUint,
}

struct ElliptiCurve {
    pub a: BigInt,
    pub b: BigInt,
    pub p: BigInt,
}
impl Point {
    fn generator() -> Point {
        Point {
            x: BigUint::parse_bytes(
                b"55066263022277343669578718895168534326250603453777594175500187360389116729240",
                10,
            )
            .unwrap(),
            y: BigUint::parse_bytes(
                b"32670510020758816978083085130507043184471273380659243275938904335757337482424",
                10,
            )
            .unwrap(),
        }
    }
}
