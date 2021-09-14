#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Non-threadsafe container for keeping autoincrement counter
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AutoIncrement<T>(T);

impl<T: Incremental> AutoIncrement<T> {
    #[allow(clippy::should_implement_trait)]
    pub fn pull(&mut self) -> T {
        let next = Incremental::get_next(&self.0);
        std::mem::replace(&mut self.0, next)
    }

    pub fn init_with(initial_value: T) -> Self {
        Self(initial_value)
    }
}

/// Trait for implementing over non-threadsafe incrementable types
pub trait Incremental: Sized {
    fn initial() -> Self;

    fn get_next(current: &Self) -> Self;

    fn init() -> AutoIncrement<Self> {
        AutoIncrement(Self::initial())
    }

    fn init_with(value: Self) -> AutoIncrement<Self> {
        AutoIncrement(value)
    }

    fn init_from(self) -> AutoIncrement<Self> {
        Self::init_with(self)
    }
}

#[cfg(test)]
mod tests {
    use crate as autoincrement;
    #[cfg(feature = "derive")]
    use autoincrement::Incremental;
    #[cfg(not(feature = "derive"))]
    use autoincrement_derive::Incremental;

    #[test]
    #[cfg(feature = "sync")]
    fn test_sync_u8() {
        #[derive(Incremental, Debug, PartialEq, Eq)]
        struct MyID(u8);

        let mut counter = MyID::init();

        assert_eq!(counter.pull(), MyID(1));
        assert_eq!(counter.pull(), MyID(2));
        assert_eq!(counter.pull(), MyID(3));
    }

    #[test]
    #[cfg(feature = "sync")]
    fn test_sync_u16() {
        #[derive(Incremental, Debug, PartialEq, Eq)]
        struct MyID(u16);

        let mut counter = MyID::init();

        assert_eq!(counter.pull(), MyID(1));
        assert_eq!(counter.pull(), MyID(2));
        assert_eq!(counter.pull(), MyID(3));
    }

    #[test]
    #[cfg(feature = "sync")]
    fn test_sync_u32() {
        #[derive(Incremental, Debug, PartialEq, Eq)]
        struct MyID(u32);

        let mut counter = MyID::init();

        assert_eq!(counter.pull(), MyID(1));
        assert_eq!(counter.pull(), MyID(2));
        assert_eq!(counter.pull(), MyID(3));
    }

    #[test]
    #[cfg(feature = "sync")]
    fn test_sync_u64() {
        #[derive(Incremental, Debug, PartialEq, Eq)]
        struct MyID(u64);

        let mut counter = MyID::init();

        assert_eq!(counter.pull(), MyID(1));
        assert_eq!(counter.pull(), MyID(2));
        assert_eq!(counter.pull(), MyID(3));
    }

    #[test]
    #[cfg(feature = "sync")]
    fn test_sync_usize() {
        #[derive(Incremental, Debug, PartialEq, Eq)]
        struct MyID(usize);

        let mut counter = MyID::init();

        assert_eq!(counter.pull(), MyID(1));
        assert_eq!(counter.pull(), MyID(2));
        assert_eq!(counter.pull(), MyID(3));
    }
}
