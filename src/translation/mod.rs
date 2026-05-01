use crate::struct_string;

struct Translation{
    sentence_data: self::struct_string::StringData,
}


#[derive(Debug)]
struct ApiCallErr;

impl Translation{
    unsafe fn new(sentence_data: self::struct_string::StringData) -> Self {
        Self { sentence_data }
    }

    // @TODO tokio
    async fn commuicate_with_translation_api(&self){ // -> self::struct_string::StringData
        // make .evn
        const API_KEY: &'static str= "";
        
        // request formating
     
        // init api request
        
        // json dezerialization?
        
        // save sentence_data
    }
}
