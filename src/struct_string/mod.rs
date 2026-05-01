use crate::string_from_bytes;

enum CONTURIES{
    ENGLISH,
    RUSSIAN,
    CHINESE,
}

pub struct StringData{
   string_data:  string_from_bytes::CString,
   contury_code: CONTURIES,
}

impl StringData{
    pub fn new(cstring_data: CString, contury_code: CONTURIES) -> Self{
        Self{ string_data, contury_code}
    }
}
