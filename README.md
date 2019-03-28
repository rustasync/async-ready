# async-ready
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Async readiness traits. Useful when implementing async state machines
that can later be wrapped in dedicated futures.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Basic usage__
```rust
#![feature(futures_api)]

use std::pin::Pin;
use std::task::{Poll, Waker};
use futures::prelude::*;
use async_ready::AsyncReady;
use std::io;

struct Fut;

impl Future for Fut {
  type Output = ();
  fn poll(self: Pin<&mut Self>, waker: &Waker) -> Poll<Self::Output> {
    Poll::Ready(())
  }
}

impl AsyncReady for Fut {
  type Ok = ();
  type Err = io::Error;

  fn poll_ready(&mut self, waker: &Waker)
    -> Poll<Result<Self::Ok, Self::Err>> {
    Poll::Ready(Ok(()))
  }
}
```

## Installation
```sh
$ cargo add async-ready
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
None.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/async-ready.svg?style=flat-square
[2]: https://crates.io/crates/async-ready
[3]: https://img.shields.io/travis/rustasync/async-ready/master.svg?style=flat-square
[4]: https://travis-ci.org/rustasync/async-ready
[5]: https://img.shields.io/crates/d/async-ready.svg?style=flat-square
[6]: https://crates.io/crates/async-ready
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/async-ready

[releases]: https://github.com/rustasync/async-ready/releases
[contributing]: https://github.com/rustasync/async-ready/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/rustasync/async-ready/labels/good%20first%20issue
[help-wanted]: https://github.com/rustasync/async-ready/labels/help%20wanted
