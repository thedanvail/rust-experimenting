#[macro_export]
macro_rules! xdef {
    ($x: ident) => {
        let x = 42;
    };
}

pub mod xdef {

    fn foo() -> i32 {
        let mut x = 0;
        xdef!(x);
        x
    }
}