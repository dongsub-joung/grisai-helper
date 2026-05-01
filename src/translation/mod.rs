use crate::struct_string;

struct Translation{
    sentence_data: self::struct_string::StringData,
}

struct TranslatedStrings{
    log_data: Vec<CString>,
}
pub impl TranslatedStrings{
    pub fn new() -> Self{ Self {log_data: Vec::from(CString::new())} }

    pub unsafe fn logging_history(&mut self, translated_cstring: Translation) -> &mut Self{
        match self.log_data.capacity(){
            0..30 => { 
                self.log_data.push(translated_cstring.sentence_data);
            },
            _ => {
               // @TODO save sentence_Data on log.txt
            }
        }

        self
    }
    pub unsafe fn get_data() -> &self::sentence_data{
        self.sentence_data
    }
}


#[derive(Debug)]
struct ApiCallErr;

pub impl Translation{
    unsafe fn new(sentence_data: self::struct_string::StringData) -> Self {
        Self { sentence_data }
    }

    // @TODO tokio
    pub async fn commuicate_with_translation_api(&self){ // -> self::struct_string::StringData
        // make .evn
        const API_KEY: &'static str= "";
        
        // request formating
     
        // init api request
        
        // json dezerialization?
       
        // convert CString

        // save sentence_data
    }
}
