#![allow(unused_variables)]
#![deny(dead_code)]

enum Enum1 {
    Variant1(isize),
    Variant2 //~ ERROR: variant is never constructed
}

enum Enum2 {
    Variant3(bool),
    #[allow(dead_code)]
    Variant4(isize),
    Variant5 { _x: isize }, //~ ERROR: variant is never constructed: `Variant5`
    Variant6(isize), //~ ERROR: variant is never constructed: `Variant6`
    _Variant7,
    Variant8 { _field: bool },
    Variant9,
    Variant10(usize)
}

impl Enum2 {
    fn new_variant8() -> Enum2 {
        Self::Variant8 { _field: true }
    }

    fn new_variant9() -> Enum2 {
        Self::Variant9
    }

    fn new_variant10() -> Enum2 {
        Self::Variant10(10)
    }
}

enum Enum3 { //~ ERROR: enum is never used
    Variant8,
    Variant9
}

fn main() {
    let v = Enum1::Variant1(1);
    match v {
        Enum1::Variant1(_) => (),
        Enum1::Variant2 => ()
    }
    let _ = Enum2::Variant3(true);
    let _ = Enum2::new_variant8();
    let _ = Enum2::new_variant9();
    let _ = Enum2::new_variant10();
}
