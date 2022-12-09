#![feature(generators)]

use futures_async_stream::stream;

fn assert_send<T: Send>(_: T) {}
fn assert_sync<T: Send>(_: T) {}

#[stream(item = i32)]
pub async fn unboxed() {
    yield 0;
}

#[stream(boxed, item = i32)]
pub async fn boxed() {
    yield 0;
}

#[stream(boxed_local, item = i32)]
pub async fn boxed_local() {
    yield 0;
}

fn main() {
    assert_send(unboxed());
    assert_sync(unboxed());
    assert_send(boxed());
    assert_send(boxed_local()); //~ ERROR `dyn futures_core::stream::Stream<Item = i32>` cannot be sent between threads safely
}
