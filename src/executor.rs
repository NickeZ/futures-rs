//! Executors
//!
//! This module contains tools for managing the raw execution of futures,
//! which is needed when building *executors* (places where futures can run).
//!
//! More information about executors can be [found online at tokio.rs][online].
//!
//! [online]: https://tokio.rs/docs/going-deeper-futures/tasks/

#[allow(deprecated)]
#[cfg(feature = "use_std")]
pub use task_impl::{Unpark, Executor, Run};

#[cfg(feature = "use_std")]
pub use task_impl::{Wait, with_wait};
pub use task_impl::{Spawn, spawn, Notify, UnsafeNotify, NotifyHandle};
