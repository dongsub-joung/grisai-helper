use thiserror::Error;

#[derive(Debug, Error)]
pub enum TranslationErr {
	#[error("translation buffer is empty")]
	BuffEmpty,

	#[error("translation API request failed: {0}")]
	ApiRequest(String),

	#[error("invalid translation API response: {0}")]
	ApiResponse(String),

	#[error("failed to convert string to CString")]
	CStringConvertErr(#[from] std::ffi::NulError),

	#[error("failed to save translation history")]
	HistorySavingErr(#[from] std::io::Error),
}

pub struct Translation{
	data: CString,
	ptr_history_buff: &BOX<Vec<CString>>
}

impl Translation {
    pub unsafe fn logging_history(&mut self, translated_string: String) -> &mut Self{
        let v_history=  match self.history_buff_ptr{
            Some(v_history) => v_history,
            None => panic!("history parsing err") // @TODO fix pacnic to custom err
        };
        match self.v_history.capacity(){
            0..30 => { 
                self.ptr_history_buff.push(translated_string);
            },
            _ => {
               // @TODO save sentence_Data on log.txt
            }
        }

        self
    }
}

pub struct ApiTranslation{
	traslated_string: String,
	
}

impl ApiTranslation{
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

fn main(){
	let mut history_buff: Box<Vec<CString>> = Box::new(Vec::new());
	let translation= Translation::new(data, &history_buff);


}

