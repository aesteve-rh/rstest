use rstest::*;

struct A;
struct B;
#[derive(Debug)]
struct D;

#[fixture]
fn fu32() -> u32 { 42 }
#[fixture]
fn fb() -> B { B {} }
#[fixture]
fn fd() -> D { D {} }
#[fixture]
fn fa() -> A { A {} }


#[rstest(
::trace::notrace(fa,fb))
]
fn single_fail(fu32: u32, fa: A, fb: B, fd: D) {
    assert!(false);
}

#[rstest(a,b,d,
    case(A{}, B{}, D{})
    ::trace::notrace(a,b))
]
fn should_fail(fu32: u32, a: A, b: B, d: D) {
    assert!(false);
}
