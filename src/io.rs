//! Provides Readers and Writers for messages.
//!
//! These should only be used in tests.

use std::io;

#[cfg(feature = "ndjson")]
pub use ndjson::NdjsonWriter;

/// Trait to write [`Message`]s contained in an [`Envelope`].
///
/// [`Envelope`]: ../struct.Envelope.html
/// [`Message`]: ../enum.Message.html
pub trait MessageWriter {
    /// Writes the message that is contained in the given envelope.
    fn write(&mut self, envelope: &crate::Envelope) -> io::Result<()>;

    /// Flushes the writer to make sure every message has been written completely.
    fn flush(&mut self) -> io::Result<()>;
}

#[cfg(feature = "ndjson")]
mod ndjson {
    use std::io;
    use std::io::Error;

    use super::MessageWriter;

    /// Newline delimited json writer.
    ///
    /// Requires feature `ndjson`.
    pub struct NdjsonWriter<W: io::Write> {
        writer: W,
    }

    impl<W: io::Write> NdjsonWriter<W> {
        /// Creates a new newline-delimited json writer that writes to the given sink.
        pub fn new(writer: W) -> NdjsonWriter<W> {
            NdjsonWriter { writer }
        }
    }

    impl<W: io::Write> MessageWriter for NdjsonWriter<W> {
        /// Writes the given message in minimal json format followed by a newline character.
        fn write(&mut self, envelope: &crate::Envelope) -> io::Result<()> {
            let message = envelope.message.as_ref().unwrap();
            match serde_json::to_writer(&mut self.writer, message) {
                Ok(()) => {}
                Err(error) => {
                    return Err(io::Error::new(io::ErrorKind::Other, error));
                }
            }
            match self.writer.write_all(b"\n") {
                Ok(()) => {}
                Err(error) => {
                    return Err(io::Error::new(io::ErrorKind::Other, error));
                }
            }
            Ok(())
        }

        /// Flushes the underlying writer to make sure every message has been written completely.
        fn flush(&mut self) -> Result<(), Error> {
            self.writer.flush()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::ast::{Feature, GherkinDocument, Location};
        use crate::{Envelope, Message};

        #[test]
        fn test_write_two_simple_gherkin_documents() {
            let gherkin_document1 = GherkinDocument {
                uri: String::from("testdata/good/empty.feature"),
                feature: None,
                comments: Vec::new(),
            };
            let gherkin_document2 = GherkinDocument {
                uri: String::from("testdata/good/incomplete_feature_1.feature"),
                feature: Some(Feature {
                    location: Some(Location { line: 1, column: 1 }),
                    language: String::from("en"),
                    keyword: String::from("Feature"),
                    name: String::from("Just a description"),
                    description: String::from("  A short description"),
                    tags: Vec::new(),
                    children: Vec::new(),
                }),
                comments: Vec::new(),
            };

            let mut output = Vec::new();
            let mut ndjson_writer = NdjsonWriter::new(&mut output);
            ndjson_writer
                .write(&create_envelope(gherkin_document1))
                .unwrap();
            ndjson_writer
                .write(&create_envelope(gherkin_document2))
                .unwrap();
            ndjson_writer.flush().unwrap();

            let written_messages = String::from_utf8(output).unwrap();
            assert_eq!(
                &written_messages,
                r###"{"gherkinDocument":{"uri":"testdata/good/empty.feature"}}
{"gherkinDocument":{"uri":"testdata/good/incomplete_feature_1.feature","feature":{"location":{"line":1,"column":1},"language":"en","keyword":"Feature","name":"Just a description","description":"  A short description"}}}
"###
            );
        }

        fn create_envelope(gherkin_document: GherkinDocument) -> Envelope {
            Envelope {
                message: Some(Message::GherkinDocument(gherkin_document)),
            }
        }
    }
}
