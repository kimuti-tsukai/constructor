#[constructor::constructor]
#[derive(Debug, PartialEq, Eq, Clone)]
struct Test {
    a: String,
    b: i32,
}

#[test]
fn test() {
    let a = Test(String::from("Hello"), 100);

    assert_eq!(a, Test { a: String::from("Hello"), b: 100 });

    dbg!(a);
}

#[constructor::constructor]
#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
struct TupleTest(String, i32);

#[constructor::constructor]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
struct UnitTest;
