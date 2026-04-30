use crate::string_from_bytes;

enum CONTURIES{
    ENGLISH,
    RUSSIAN,
    CHINESE,
}

pub struct StringData{
   string_data:  string_from_bytes::CString,
   contury_code: CONTURIES,
   translated_data: String, // IDK Vec<CString> (I can converte Vec<String>?)
}

impl StringData{
    pub fn new(cstring_data: CString, contury_code: CONTURIES) -> Self{
        Self{ string_data, contury_code, translated_data: String::new() }
    }

    pub fn save_translated_data(&mut self, translated_data: String){
       self.translated_data= translated_data; 
    }
}
