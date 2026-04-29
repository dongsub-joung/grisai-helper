enum CONTURIES{
    ENGLISH,
    RUSSIAN,
    CHINESE,
}

struct StringData{
   string_data: String,
   contury_code: CONTURIES,
   translated_data: String,
}

impl StringData{
    pub fn new(string_data: String, contury_code: CONTURIES) -> Self{
        Self{ string_data, contury_code, translated_data: String::new() }
    }

    pub fn save_translated_data(&mut self, translated_data: String){
       self.translated_data= translated_data; 
    }
}


