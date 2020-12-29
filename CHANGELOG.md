# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## [Unreleased]

## [0.2.3] - 2020-11-08

- Update `pin-project` to 1.0.

## [0.2.2] - 2020-06-11

- All macros can now compile with `forbid(unsafe_code)`.

- Support overlapping lifetime names in HRTB.

- Diagnostic improvements.

## [0.2.1] - 2020-05-21

- Diagnostic improvements.

## [0.2.0] - 2020-05-10

- [Renamed `#[async_stream]` to `#[stream]`.][45]

- [Renamed `#[async_try_stream]` to `#[stream]`.][45]

- [Renamed `async_stream_block!` to `stream_block!`.][45]

- [Renamed `async_try_stream_block!` to `try_stream_block!`.][45]

- Bug fixes.

- Diagnostic improvements.

[45]: https://github.com/taiki-e/futures-async-stream/pull/45

## [0.1.5] - 2020-05-04

- [`#[async_stream]` and `#[async_try_stream]` attributes can now be used on async blocks.][44]

[44]: https://github.com/taiki-e/futures-async-stream/pull/44

## [0.1.4] - 2020-04-22

- [futures-async-stream now works on no-std environment.][34]

[34]: https://github.com/taiki-e/futures-async-stream/pull/34

## [0.1.3] - 2020-02-20

- [Fixed build failure on latest nightly.][33]

[33]: https://github.com/taiki-e/futures-async-stream/pull/33

## [0.1.2] - 2019-12-10

- [Fixed build failure on latest nightly.][31]

[31]: https://github.com/taiki-e/futures-async-stream/pull/31

## [0.1.1] - 2019-11-13

- Fixed duplicate documentation.

## [0.1.0] - 2019-11-13

- Updated `futures` to 0.3.0.

## [0.1.0-alpha.7] - 2019-09-25

- Updated `pin-project` to 0.4.

## [0.1.0-alpha.6] - 2019-09-06

- [Added `async_try_stream` to support `?` operator in async stream.][15] e.g. You can write the following:

  ```rust
  #![feature(generators)]
  use futures::stream::Stream;
  use futures_async_stream::async_try_stream;

  #[async_try_stream(ok = i32, error = Box<dyn std::error::Error + Send + Sync>)]
  async fn foo(stream: impl Stream<Item = String>) {
      #[for_await]
      for x in stream {
          yield x.parse()?;
      }
  }
  ```

- Updated `pin-project` to 0.4.0-alpha.9.

[15]: https://github.com/taiki-e/futures-async-stream/pull/15

## [0.1.0-alpha.5] - 2019-08-29

- Pinned the version of pin-project dependency.

## [0.1.0-alpha.4] - 2019-08-23

- Removed usage of some feature gates.

- Updated `pin-project` to 0.4.0-alpha.4.

## [0.1.0-alpha.3] - 2019-08-14

- Updated `proc-macro2`, `syn`, and `quote` to 1.0.

## [0.1.0-alpha.2] - 2019-08-07

- [You can now use async stream functions in traits.][12] e.g. You can write the following:

  ```rust
  #![feature(async_await, generators)]
  use futures_async_stream::async_stream;

  trait Foo {
      #[async_stream(boxed, item = u32)]
      async fn method(&mut self);
  }

  struct Bar(u32);

  impl Foo for Bar {
      #[async_stream(boxed, item = u32)]
      async fn method(&mut self) {
          while self.0 < u32::max_value() {
              self.0 += 1;
              yield self.0;
          }
      }
  }
  ```

[12]: https://github.com/taiki-e/futures-async-stream/pull/12

## [0.1.0-alpha.1] - 2019-07-31

Initial release

[unreleased]: https://github.com/taiki-e/futures-async-stream/compare/v0.2.3...HEAD
[0.2.3]: https://github.com/taiki-e/futures-async-stream/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/taiki-e/futures-async-stream/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/futures-async-stream/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.0-alpha.7...v0.1.0
[0.1.0-alpha.7]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.0-alpha.6...v0.1.0-alpha.7
[0.1.0-alpha.6]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.0-alpha.5...v0.1.0-alpha.6
[0.1.0-alpha.5]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.0-alpha.4...v0.1.0-alpha.5
[0.1.0-alpha.4]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.0-alpha.3...v0.1.0-alpha.4
[0.1.0-alpha.3]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.0-alpha.2...v0.1.0-alpha.3
[0.1.0-alpha.2]: https://github.com/taiki-e/futures-async-stream/compare/v0.1.0-alpha.1...v0.1.0-alpha.2
[0.1.0-alpha.1]: https://github.com/taiki-e/futures-async-stream/releases/tag/v0.1.0-alpha.1
