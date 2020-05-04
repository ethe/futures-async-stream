#![feature(generators, stmt_expr_attributes, proc_macro_hygiene)]

use futures_async_stream::{for_await, stream};

#[stream(item = i32)]
async fn stream(x: i32) {
    for i in 1..=x {
        yield i
    }
}

async fn async_fn() {
    for _i in 1..2 {
        async {}.await?; //~ ERROR the `?` operator can only be applied to values that implement `std::ops::Try`
    }
}

#[stream(item = i32)]
async fn async_stream_fn() {
    for _i in 1..2 {
        async {}.await?; //~ ERROR the `?` operator can only be applied to values that implement `std::ops::Try`
    }
}

// FIXME: this is a compiler bug and probably the fix will require https://github.com/rust-lang/rust/issues/43081
async fn async_fn_and_for_await() {
    #[for_await]
    //~^ ERROR the `?` operator can only be applied to values that implement `std::ops::Try`
    for _i in stream(2) {
        async {}.await?;
    }
}

// FIXME: this is a compiler bug and probably the fix will require https://github.com/rust-lang/rust/issues/43081
#[stream(item = i32)]
async fn async_stream_fn_and_for_await() {
    #[for_await]
    //~^ ERROR the `?` operator can only be applied to values that implement `std::ops::Try`
    for _i in stream(2) {
        async {}.await?;
    }
}

fn main() {}
