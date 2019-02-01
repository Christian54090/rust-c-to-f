fn main() {

}

fn c_to_f(num: i32) -> i32 {
    num * 2 + 30
}

fn f_to_c(num: i32) -> i32 {
    (num - 30) / 2
}

#[test]
fn test_c_to_f() {
    assert_eq!(52, c_to_f(11));
}

#[test]
fn test_f_to_c() {
    assert_eq!(19, f_to_c(68));
}