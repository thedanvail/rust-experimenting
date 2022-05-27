pub mod avec {
    #[macro_export]
    macro_rules! avec {
        () => {};

        // Introducing args and types
        // ty is a Type
        ($arg1: ty, $arg2: expr, $arg3: path) => {};

        // We can also introduce whatever we like as syntax
        // so long as it is valid Rust.
        // so we have a type, an arrow, an expression, a semicolon, and a path.
        ($arg1: ty => $arg2: expr; $arg3: path) => {};

        // This one is for the purposes of testing `cargo expand`
        ($arg1:ty => $arg2:ident) => {
            type $arg2 = $arg1;
        };

        // Something like `typedef` would use the following:
        ($arg1: ident as $arg2: ty) => {
            type $arg1 = $arg2;
        }
    }

    // Allow the test to see the avec macro
    // This will allow code in the same crate to see the macro
    // but `#[macro_export] will allow other crates to see it.
    pub(crate) use avec;

    // The following three are valid for the expression match `()`
    avec!();
    avec![];
    avec!{}

    // The following is valid for the third macro signature
    // of type, arrow operator, expression, semicolon, and path.
    avec!(
        // While this is syntactically valid and can be parsed,
        // this is not valid Rust.
        // So we can take liberty with our inputs (within reason);
        // however, the output MUST be valid Rust.
        u32 => x.foo(); std::path
    );

    // This should inject `type also32 = u32;` with `cargo expand`
    avec!(u32 => Alsou32);
    avec!(Alsoalsou32 as u32);

}

#[test]
fn avec_no_pattern_no_return() {
    avec::avec!();
}
