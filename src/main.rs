mod translation;
mod string_from_bytes;
mod struct_string;
mod bytes_data;

use ilhook::{HookError, x86::{CallbackOption, HookFlags, HookPoint, HookType, Hooker, Registers}};
use core::panic;
use pelite::{Result};
use pelite::pe64::{Pe, PeFile};
use pelite::resources::FindError;

// ilhook
unsafe extern "cdecl" fn on_check_sn(reg:*mut Registers, _:usize){
    println!("machine_hash: {}, sn_hash: {}", (*reg).ebx, (*reg).eax);
    (*reg).eax = (*reg).ebx; //we modify the sn_hash!
}

#[derive(Debug)]
struct ConvertingErr;

// pelite
// @TODO Fix Generic type problem
fn get_bytes_data<'a>(file: PeFile<'a>) -> Option<Vec<Box<bytes_data::BytesData>>> {
    const DATA_PATH: &'static str= "";

	// Access the resources
	let resources = file.resources()?;
    
    // get enum Name from Resources 
    // if match name = grisaia (Japanese) -> keep going, No -> _ 

	// Find the desired resource by its path
	let data = match resources.find_data(DATA_PATH){
        Ok(_data_entry) => {
            _data_entry
        },
        Err(e) => {
            panic!("failed to find data");
        }
    };
	
    // @TODO add custom Error type
    let manifest = data.bytes();
    
    // @TODO need to get other data
    // pelite::resources -> pub fn new(section: &'a [u8], dir: &'a IMAGE_DATA_DIRECTORY) -> Resources<'a>

    let v_manifests: Vec<Boxc<bytes_data::BytesData>>= Vec::from(Box::from((manifest));
	
    Option::Some(v_manifests)
}

fn main(){
    // searching memory location -> CPP code

    // hooking -> ihook & custom
    let hooker= Hooker::new(
        0x40107F,  // from CPP
        HookType::JmpBack(on_check_sn), 
        CallbackOption::None, //impl !Sync for CallbackOption 
        0, 
        HookFlags::empty());
    
    //check_serial_number();
    
    unsafe{
        let result_hook= hooker.hook();
        let mut hook_point= match result_hook {
            Ok(_hook_point) =>{
                _hook_point
            },
            Err(e) =>{
                panic!("failed hook");
            }
        };
        
        // @TODO when japanese show up in game, try to capture a dll file
        // Crate pedum
        
        // @TODO extract Japanese -> pelite(IDK its working?)
        let pe_file;
        let v_bytes_data= match get_bytes_data(pe_file){
            Some(_bytes) =>{
                return _bytes;
            },
            None => { 
                panic!("None value") 
            }
        };

        // init translate
        let japanese_data= &v_bytes_data.iter().map(|data| 
                match data.source{
                    String::from("japanese") => data
                }
            );
        let cstring_data= string_from_bytes::CString::from_bytes(japanese_data);

        // when thread will move or targeted process will quite
        // if  { 
        //     HookPoint::unhook(running_hook); 
        // }
    };

    // Options
    // 1. hotkey is "s"
}

fn auto_skipping(){ // pressing cnt key and hold
    // if press "s" key exit 
}
