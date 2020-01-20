//! IdGenerator trait and some implementations.
//!
//! IdGenerators for the ids in [`GherkinDocument`], [`Pickle`], etc.
//!
//! [`GherkinDocument`]: ../ast/struct.GherkinDocument.html
//! [`Pickle`]: ../pickle/struct.Pickle.html

/// Trait to generate new identifiers of type `String`.
pub trait IdGenerator {
    /// Generates a new identifier.
    fn new_id(&mut self) -> String;
}

/// IdGenerator that creates incrementing numbers (0, 1, 2, ...).
#[derive(Default)]
pub struct IncrementingIdGenerator {
    next: usize,
}

impl IncrementingIdGenerator {
    /// Creates a new IncrementingIdGenerator starting from 0.
    pub fn new() -> IncrementingIdGenerator {
        IncrementingIdGenerator::default()
    }
}

impl IdGenerator for IncrementingIdGenerator {
    fn new_id(&mut self) -> String {
        let id = self.next.to_string();
        self.next += 1;
        id
    }
}

#[cfg(feature = "uuid_v4")]
pub use uuid_v4::UuidGenerator;

#[cfg(feature = "uuid_v4")]
mod uuid_v4 {
    use super::IdGenerator;

    /// IdGenerator that creates version 4 uuids.
    ///
    /// Requires feature `uuid_v4`.
    #[derive(Default)]
    pub struct UuidGenerator {}

    impl UuidGenerator {
        /// Creates a new UuidGenerator.
        pub fn new() -> UuidGenerator {
            UuidGenerator::default()
        }
    }

    impl IdGenerator for UuidGenerator {
        fn new_id(&mut self) -> String {
            uuid::Uuid::new_v4().to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incrementing() {
        let mut id_generator = IncrementingIdGenerator::new();

        let id1 = id_generator.new_id();
        assert_eq!(&id1, "0");

        let id2 = id_generator.new_id();
        assert_eq!(&id2, "1");
    }

    #[test]
    #[cfg(feature = "uuid_v4")]
    fn test_uuid() {
        let mut id_generator = UuidGenerator::new();

        let id1 = id_generator.new_id();
        assert_eq!(id1.chars().count(), 36);
        assert_eq!(id1.chars().filter(|c| c.is_alphanumeric()).count(), 32);
        assert_eq!(id1.chars().filter(|c| *c == '-').count(), 4);

        let id2 = id_generator.new_id();
        assert_ne!(id1, id2);
        assert_eq!(id2.chars().count(), 36);
        assert_eq!(id2.chars().filter(|c| c.is_alphanumeric()).count(), 32);
        assert_eq!(id2.chars().filter(|c| *c == '-').count(), 4);
    }
}
