    use std::collections::{vec_deque, VecDeque};
    use std::io::{BufReader, Bytes, Cursor, Read, Seek};
    use std::str::from_utf8;

    // This struct is responsible for reading the data to be parsed as JSON
    // It also provides an iterator to iterate over the data character by character.
    pub struct JsonReader<T>
    where
        T: Read + Seek,
    {
        //Reference to the input data, that implements READ
        reader : BufReader<T>,

        // A character buffer that holds queue of characters to be processed by the iterator
        // This is because UTF-8 characters can take a at-most of 4-bytes of data
        // The reader always reads 4-bytes of the data

        // We then iterate over the in the form of characters, regardless if the take
        // 1, 2, 3 or 4 bytes

        // Using VecDeque, because we need to read characters in FIFO manner
        // As they are processed
        character_buffer : VecDeque<char>
    }

    impl<T> JsonReader<T>
    where
        T: Read + Seek,
    {
        ///Creating a new JsonReader that reads from a File
        /// # Examples
        ///
        /// ```
        /// use std::fs::File;
        /// use std::io::BufReader;
        /// use json_parser::reader::JsonReader;
        ///
        /// let file = File::create("dummy.json").unwrap();
        /// let reader = BufReader(file);
        ///
        /// let json_reader = JsonReader::new(reader);
        /// ```

        pub fn new(reader : BufReader<T>) -> Self{
            Self{
                reader,
                character_buffer: VecDeque::with_capacity(4)
            }
        }

        /// Create a new [`JsonReader`] that reads from a given byte stream
        ///
        /// # Examples
        ///
        /// ```
        /// use std::io::{BufReader, Cursor};
        /// use json_parser::reader::JsonReader;
        ///
        /// let input_json_string = r#"{"key1":"value1","key2":"value2"}"#;
        ///
        /// let json_reader = JsonReader::<Cursor<&'static [u8]>>::from_bytes(input_json_string.as_bytes());
        /// ```

        #[must_use]
        pub fn from_bytes(bytes: &[u8]) -> JsonReader<Cursor<&[u8]>>{
            JsonReader{
                reader : BufReader::new(Cursor::new(bytes)),
                character_buffer : VecDeque::with_capacity(4)
            }
        }
    }