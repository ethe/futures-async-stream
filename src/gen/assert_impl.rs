// This file is @generated by futures-async-stream-internal-codegen
// (gen_assert_impl function at tools/codegen/src/main.rs).
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
#![allow(clippy::std_instead_of_alloc, clippy::std_instead_of_core)]
#[allow(dead_code)]
fn assert_send<T: ?Sized + Send>() {}
#[allow(dead_code)]
fn assert_sync<T: ?Sized + Sync>() {}
#[allow(dead_code)]
fn assert_unpin<T: ?Sized + Unpin>() {}
#[allow(dead_code)]
fn assert_unwind_safe<T: ?Sized + std::panic::UnwindSafe>() {}
#[allow(dead_code)]
fn assert_ref_unwind_safe<T: ?Sized + std::panic::RefUnwindSafe>() {}
const _: fn() = || {
    assert_send::<crate::future::ResumeTy>();
    assert_sync::<crate::future::ResumeTy>();
    assert_unpin::<crate::future::ResumeTy>();
    assert_unwind_safe::<crate::future::ResumeTy>();
    assert_ref_unwind_safe::<crate::future::ResumeTy>();
};
