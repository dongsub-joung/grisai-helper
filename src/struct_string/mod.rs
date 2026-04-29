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
        self{ string_data, contury_code, translated_data: String::new() }
    }


}
