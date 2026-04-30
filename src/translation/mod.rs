use crate::struct_string;

struct Translation{
    sentence_data: self::struct_string::StringData,
}

impl Translation{
    fn new(sentence_data: self::struct_string::StringData) -> Self {
        Self { sentence_data }
    }
}
