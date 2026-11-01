//! Outlives bounds on associated types and projections, checked against rustc.

use a_mir_formality::{crates, FormalityTest};
use formality_core::test;

// An impl whose associated type is `'static` satisfies `type Item: 'static`.
#[test]
fn assoc_type_impl_satisfies_static_bound() {
    FormalityTest::new(crates![crate Foo {
        trait A { type Item: ['static]; }
        impl A for u32 { type Item = u32; }
    }])
    .skip_execute()
    .rustc_ok()
    .ok();
}

// An impl whose associated type borrows for `'a` does not satisfy `type Item: 'static`.
#[test]
fn assoc_type_impl_violates_static_bound() {
    FormalityTest::new(crates![crate Foo {
        trait A { type Item: ['static]; }
        impl<'a> A for &'a u32 { type Item = &'a u32; }
    }])
    .skip_execute()
    .rustc_err(expect_test::expect![[r#"
        error[E0477]: the type `&'a00 u32` does not fulfill the required lifetime
         --> lib.rs
          |
        6 |     type Item = &'a00 u32;
          |                 ^^^^^^^^^
          |
        note: type must satisfy the static lifetime as required by this binding
         --> lib.rs
          |
        2 |     type Item: 'static;
          |                ^^^^^^^

        For more information about this error, try `rustc --explain E0477`.
        error: could not compile `Foo` (lib) due to 1 previous error
    "#]])
    .err(expect_test::expect!["crates/formality-rust/src/prove/prove_outlives.rs:8:1: no applicable rules for prove_outlives { a: !lt_0, b: ' static, assumptions: {}, env: Env { variables: [!lt_0], bias: Soundness, pending: [], allow_pending_outlives: false } }"]);
}

// A type parameter bounded by a lifetime in a where clause.
#[test]
fn fn_type_param_outlives_where_clause() {
    FormalityTest::new(crates![crate Foo {
        fn f<'a, T>(x: &'a T) -> () where T: 'a { trusted }
    }])
    .skip_execute()
    .rustc_ok()
    .ok();
}

// An outlives bound on an associated type projection in a where clause.
#[test]
fn fn_projection_outlives_where_clause() {
    FormalityTest::new(crates![crate Foo {
        trait A { type Item: []; }
        fn f<'a, T>() -> () where T: A, <T as A>::Item: 'a { trusted }
    }])
    .skip_execute()
    .rustc_ok()
    .ok();
}
