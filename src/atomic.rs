use std::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Threadsafe container for keeping autoincrement counter
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AsyncIncrement<T: AsyncIncremental>(T::Atomic);

impl<T: AsyncIncremental> AsyncIncrement<T> {
    #[allow(clippy::should_implement_trait)]
    pub fn pull(&self) -> T {
        AsyncIncremental::get_next(&self.0)
    }

    pub fn init_with(initial_value: T) -> Self {
        Self(AsyncIncremental::into_atomic(initial_value))
    }
}

/// Trait for implementing over threadsafe incrementable types
pub trait AsyncIncremental: Sized {
    type Atomic: Atomic;

    fn initial() -> Self;

    fn get_next(atomic: &Self::Atomic) -> Self;

    fn into_atomic(value: Self) -> Self::Atomic;

    fn init() -> AsyncIncrement<Self> {
        Self::init_with(Self::initial())
    }

    fn init_with(value: Self) -> AsyncIncrement<Self> {
        AsyncIncrement(Self::into_atomic(value))
    }

    fn init_from(self) -> AsyncIncrement<Self> {
        Self::init_with(self)
    }
}

/// Only for type-safe purposes. You don't need to use this trait.
pub trait Atomic: Send + Sync + std::fmt::Debug {
    type Inner: Copy;

    fn new(initial_value: Self::Inner) -> Self;

    fn next(&self, step: Self::Inner) -> Self::Inner;
}

macro_rules! impl_atomic {
    ($basic:ty => $atomic:ty) => (
        impl Atomic for $atomic {
            type Inner = $basic;

            fn new(initial_value: Self::Inner) -> Self {
                Self::new(initial_value)
            }

            fn next(&self, step: Self::Inner) -> Self::Inner {
                self.fetch_add(step, std::sync::atomic::Ordering::SeqCst)
            }
        }
    );

    ($basic:ty => $atomic:ty, $($basics:ty => $atomics:ty),+) => (
        impl_atomic!($basic => $atomic);
        impl_atomic!($($basics => $atomics),+);
    )
}

impl_atomic!(
    u8 => AtomicU8,
    u16 => AtomicU16,
    u32 => AtomicU32,
    u64 => AtomicU64,
    usize => AtomicUsize
);

#[cfg(test)]
mod tests {
    use crate as autoincrement;

    #[cfg(feature = "derive")]
    use autoincrement::AsyncIncremental;
    #[cfg(not(feature = "derive"))]
    use autoincrement_derive::AsyncIncremental;

    #[test]
    #[cfg(feature = "async")]
    fn test_async_u8() {
        #[derive(AsyncIncremental, Debug, PartialEq, Eq)]
        struct MyID(u8);

        let counter = MyID::init();

        assert_eq!(counter.pull(), MyID(1));
        assert_eq!(counter.pull(), MyID(2));
        assert_eq!(counter.pull(), MyID(3));
    }

    #[test]
    #[cfg(feature = "async")]
    fn test_async_u16() {
        #[derive(AsyncIncremental, Debug, PartialEq, Eq)]
        struct MyID(u16);

        let counter = MyID::init();

        assert_eq!(counter.pull(), MyID(1));
        assert_eq!(counter.pull(), MyID(2));
        assert_eq!(counter.pull(), MyID(3));
    }

    #[test]
    #[cfg(feature = "async")]
    fn test_async_u32() {
        #[derive(AsyncIncremental, Debug, PartialEq, Eq)]
        struct MyID(u32);

        let counter = MyID::init();

        assert_eq!(counter.pull(), MyID(1));
        assert_eq!(counter.pull(), MyID(2));
        assert_eq!(counter.pull(), MyID(3));
    }

    #[test]
    #[cfg(feature = "async")]
    fn test_async_u64() {
        #[derive(AsyncIncremental, Debug, PartialEq, Eq)]
        struct MyID(u64);

        let counter = MyID::init();

        assert_eq!(counter.pull(), MyID(1));
        assert_eq!(counter.pull(), MyID(2));
        assert_eq!(counter.pull(), MyID(3));
    }

    #[test]
    #[cfg(feature = "async")]
    fn test_async_usize() {
        #[derive(AsyncIncremental, Debug, PartialEq, Eq)]
        struct MyID(usize);

        let counter = MyID::init();

        assert_eq!(counter.pull(), MyID(1));
        assert_eq!(counter.pull(), MyID(2));
        assert_eq!(counter.pull(), MyID(3));
    }
}
