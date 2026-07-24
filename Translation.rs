#[derive(Debug)]
pub enum TranslationErr {
	BUFF_EMPTY,
	API_REQUEST,
	API_RESPONSE,
	CSTRING_CONVERT_ERR,
	HISTORY_SAVING_ERR
}

pub struct Translation{
	data: CString,
	history_buff: &BOX<Vec<CString>>
}

impl Translation {

}

pub struct ApiTranslation{
	traslated_string: String,
	
}

fn main(){
	let mut history_buff: BOX<Vec<CString>= Box::new(Vec::new());
	let translation= Translation::new(data, &history_buff);


}