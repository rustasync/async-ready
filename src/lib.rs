#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//!
//! ```rust
//! #![feature(futures_api)]
//!
//! use std::pin::Pin;
//! use std::task::{Poll, Waker};
//! use futures::prelude::*;
//! use std::io;
//!
//! struct Fut;
//!
//! impl Future for Fut {
//!   type Output = ();
//!   fn poll(self: Pin<&mut Self>, waker: &Waker) -> Poll<Self::Output> {
//!     Poll::Ready(())
//!   }
//! }
//!
//! impl ready::Ready for Fut {
//!   type Ok = ();
//!   type Err = io::Error;
//!   fn poll_ready(&self, waker: &Waker)
//!     -> Poll<Result<Self::Ok, Self::Err>> {
//!     Poll::Ready(Ok(()))
//!   }
//! }
//! ```

#![feature(futures_api)]

use std::task::{Poll, Waker};

/// Determine if the underlying API can be written to.
pub trait WriteReady {
  /// The type of successful values yielded by this trait.
  type Ok;

  /// The type of failures yielded by this trait.
  type Err;

  /// Check if the underlying API can be written to.
  fn poll_write_ready(&self, waker: &Waker) -> Poll<Result<Self::Ok, Self::Err>>;
}

/// Determine if the underlying API can be read from.
pub trait ReadReady {
  /// The type of successful values yielded by this trait.
  type Ok;

  /// The type of failures yielded by this trait.
  type Err;

  /// Check if the underlying API can be read from.
  fn poll_read_ready(&self, waker: &Waker) -> Poll<Result<Self::Ok, Self::Err>>;
}

/// Determine if a struct is ready to yield futures.
///
/// This is useful when a `Stream` borrows an internal struct, and the internal
/// struct is in charge of establishing the io channel. That way the stream and
/// the readiness can be decoupled.
///
/// Once the IO channel is ready, `poll_ready` should always return
/// `Poll::Ready`.
pub trait Ready {
  /// The type of successful values yielded by this trait.
  type Ok;

  /// The type of failures yielded by this trait.
  type Err;

  /// Check if the stream can be read from.
  fn poll_ready(&self, waker: &Waker) -> Poll<Result<Self::Ok, Self::Err>>;
}
