//fn main() {
//     let result_v_events = |data: String| -> Result<ChangedEvent, serde_json::Error> {
//         let v_events: ChangedEvent= serde_json::from_str(&data)?;

//         Ok(v_events)
//     };

//     let get_file_size = |json_path: &'static str| -> io::Result<u64> {
//         // metadata() fetches file information from the OS
//         let metadata = fs::metadata(json_path)?;

//         // len() returns the size in bytes
//         Ok(metadata.len())
//     };

//     let get_latest_json_lines = |json_path: &str, limit: usize| -> ChangedEvent {
//         let file = File::open(json_path).expect("Could not open file");
//         let rev_lines = RevLines::new(BufReader::new(file));
//         let mut obj_event: Option<ChangedEvent>;

//         // Take the last 'limit' lines and parse them
//         rev_lines
//             .take(limit)
//             .filter_map(|line| {
//                 let line_str = line.ok()?;
                
//                 serde_json::from_str::<ChangedEvent>(&line_str).ok()
//             }
//         );

//         // @TODO unwrap rev_lines: ReLinesMBufReader<File>>
//         for e in result_v_events {
            
//         }

//         match obj_event {
//             Some(obj) => return obj,
//             _ => panic!("Event object is null")
//         }
//     };

//     fn rotate_event_logs() -> io::Result<()> {
        
//         // 1. Generate the filename with the current date: e.g., 2026-04-26_logs.json
//         let date_str = Local::now().format("%Y-%m-%d").to_string();
//         let destination_path = format!("{}_logs.json", date_str);

//         // Check if source exists before trying to copy
//         if Path::new(JSON_PATH).exists() {
//             // 2. Copy the file to the new dated log
//             fs::copy(JSON_PATH, &destination_path)?;
//             println!("Successfully backed up to {}", destination_path);

//             // 3. Overwrite/Clear the original file (makes it 0 bytes)
//             // If you'd rather delete it entirely, use fs::remove_file(source_path)?;
//             fs::write(JSON_PATH, "")?;
//         } else {
//             println!("Source file {} does not exist. Skipping.", JSON_PATH);
//         }

//         Ok(())
//     }
//     fn fim_process_sleep_for_backup() {
//         let args: Vec<String> = env::args().collect();

//         let string_pid = &args[1];
//         let pid_raw = string_pid
//             .parse::<i32>()
//             .expect("Not a valid number. Plz input PID");

//         let pid = nix::unistd::Pid::from_raw(pid_raw);

//         match kill(pid, Signal::SIGSTOP) {
//             Ok(_) => println!("Daemon {} paused.", pid_raw),
//             Err(e) => eprintln!("Failed to pause: {}", e),
//         }

//         let result_backup= rotate_event_logs();
//         match result_backup {
//             Err(e) => { panic!("failed to backup process"); },
//             Ok(()) => { }
//         };

//         match kill(pid, Signal::SIGCONT) {
//             Ok(_) => println!("Daemon {} resumed.", pid_raw),
//             Err(e) => eprintln!("Failed to resume: {}", e),
//         }
//     }

//     // init
//     fn noting(){
//         let data: String = fs::read_to_string(JSON_PATH).expect("failed to find path");

//         let _result_data = result_v_events(data);


//         // get latest 1 Json data as struct ChangedEvent
//         let recent_events = get_latest_json_lines(JSON_PATH, 1_usize);
        
//         print_json_data_pretty(&recent_events);
        
//         // print_all_data(&_result_data);
        
//         let file_size = get_file_size(JSON_PATH).expect("failed get a log file size");
//         if file_size >= 20048000000_u64 {
//             fim_process_sleep_for_backup();
//         }
//     }
// //}

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

// pelite
// @TODO Fix Geeric type problem
fn get_bytes_data<'a>(file: PeFile<'a>) -> Result<&'a [u8], FindError> {
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
	
    let manifest = data.bytes()?;

    // need to get other data
    // pelite::resources -> pub fn new(section: &'a [u8], dir: &'a IMAGE_DATA_DIRECTORY) -> Resources<'a>

	Ok(manifest)
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
        let bytes_data= match get_bytes_data(pe_file){
            Ok(_bytes) =>{
                return _bytes;
            },
            Err(e) => { 
                panic!("failed to get bytes data"); 
            },
            _ => {}
        };

        // @TODO translatie

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
