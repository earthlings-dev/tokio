//! Synchronization primitives

mod cancellation_token;
pub use cancellation_token::{
    CancellationToken, WaitForCancellationFuture, WaitForCancellationFutureOwned, guard::DropGuard,
    guard_ref::DropGuardRef,
};
pub(crate) use cancellation_token::{RunUntilCancelledFuture, RunUntilCancelledFutureOwned};

mod mpsc;
pub use mpsc::{PollSendError, PollSender};

mod poll_semaphore;
pub use poll_semaphore::PollSemaphore;

mod reusable_box;
pub use reusable_box::ReusableBoxFuture;

#[cfg(test)]
mod tests;
