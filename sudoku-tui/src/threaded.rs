use std::thread::{self, JoinHandle};

/// Represent a value that is either being computed on some thread, or that
/// has been computed (and the corresponding thread joined).
enum ThreadedState<T> {
    Joined(T),
    Ongoing(JoinHandle<T>),
    /// This variant is only a trick to be able to call [`mem::replace`] and
    /// take ownership of the thread handle to join and get the value.
    Empty,
}

impl<T> ThreadedState<T> {
    fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        Self::Ongoing(thread::spawn(f))
    }

    /// Non-blocking attempt at joining the thread and get the desired value.
    fn try_join(&mut self) -> Option<&T> {
        if let ThreadedState::Ongoing(handle) = self {
            if handle.is_finished() {
                // Take ownership of the handle, using the Empty variant
                // as a placeholder while we join and get the actual value.
                if let ThreadedState::Ongoing(handle) =
                    std::mem::replace(self, ThreadedState::Empty)
                {
                    let value = handle.join().unwrap();
                    *self = ThreadedState::Joined(value);
                }
            }
        }
        match self {
            ThreadedState::Joined(v) => Some(v),
            ThreadedState::Ongoing(_) => None,
            ThreadedState::Empty => unreachable!(),
        }
    }
}

/// Represent a computation that is either ongoing on an external thread or
/// finished (and the corresponding thread joined).
// Opaque wrapper around the enum to avoid leaking the `ThreadedState::Empty`
// variant.
pub struct Threaded<T>(ThreadedState<T>);

impl<T> Threaded<T> {
    /// Spawn a thread and wrap it in [`Threaded`].
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        Self(ThreadedState::spawn(f))
    }

    /// Non-blocking attempt at joining the thread and recovering the result
    /// of the computation.
    pub fn try_join(&mut self) -> Option<&T> {
        self.0.try_join()
    }
}
