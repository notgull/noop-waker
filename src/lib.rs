// MIT/Apache2 License

//! A waker that does nothing when it is woken. Useful for "now or never" type scenarioes where a future is
//! unlikely to be polled more than once, or for "spinning" executors.
//! 
//! # Example
//! 
//! A very inefficient implementation of the `block_on` function that polls the future over and over.
//! 
//! ```
//! use core::{future::Future, hint::spin_loop, task::{Context, Poll}};
//! use futures_lite::future::poll_fn;
//! use noop_waker::noop_waker;
//! 
//! fn block_on<R>(f: impl Future<Output = R>) -> R {
//!     // pin the future to the stack
//!     futures_lite::pin!(f);
//! 
//!     // create the context
//!     let waker = noop_waker();
//!     let mut ctx = Context::from_waker(&waker);
//! 
//!     // poll future in a loop
//!     loop {
//!         match f.as_mut().poll(&mut ctx) {
//!             Poll::Ready(o) => return o,
//!             Poll::Pending => spin_loop(),
//!         }
//!     }
//! }
//! 
//! // this future returns pending 5 times before returning ready
//! 
//! let mut counter = 0;
//! let my_future = poll_fn(|ctx| {
//!     if counter < 5 {
//!         counter += 1;
//!         ctx.waker().wake_by_ref();
//!         Poll::Pending
//!     } else {
//!         Poll::Ready(7)
//!     }
//! });
//! 
//! assert_eq!(block_on(my_future), 7);
//! ```

#![no_std]
#![warn(clippy::pedantic)]

use core::{ptr, task::{Waker, RawWaker, RawWakerVTable}};

/// The whole point. Returns a waker that does nothing.
#[inline]
#[must_use]
pub fn noop_waker() -> Waker {
    let raw = RawWaker::new(ptr::null(), &NOOP_WAKER_VTABLE);
    
    // SAFETY: the contracts for RawWaker and RawWakerVTable are upheld
    unsafe { Waker::from_raw(raw) }
}

const NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

unsafe fn noop_clone(_p: *const ()) -> RawWaker {
    // SAFETY: this retains all of the waker's resources, of which there are none
    RawWaker::new(ptr::null(), &NOOP_WAKER_VTABLE)
}

unsafe fn noop(_p: *const ()) {}
