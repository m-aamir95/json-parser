    use std::collections::{vec_deque, VecDeque};
    use std::io::{BufReader, Cursor, Read, Seek};
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