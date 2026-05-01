pub struct CString{
    c_string_data: CString,
}

impl CString{
    pub unsafe fn from_bytes(bytes: &[u8]) -> Self{
        Self { c_string_data: unsafe { CString::from_bytes(bytes) } }
    } 
}
