// here i cant use 1 lifetime, borrow checker will complain on line println!("{}", s)
struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

#[repr(transparent)]
pub struct NewString(String);

fn main() {
    let mut s = "Hello";
    *MutStr { s: &mut s }.s = "world";
    println!("{}", s);

    macro_rules! let_foo {
        ($x:expr) => {
            let foo = $x;
        };
    }

    let foo = 1;
    let_foo!(2);
    assert_eq!(foo, 1)
}
