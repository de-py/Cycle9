use rocket::response::{status};
use rocket::http::Status;
use bindings::Windows::Win32;
use crate::mem_look::lister::{*};
use crate::mem_look::read_memory::{*};
use rocket::serde::{Serialize, json::Json};
use Win32::System::SystemServices::{HANDLE};
use rocket::State;
use std::sync::{Arc, Mutex};

struct ProcessState{
	current: Arc<Mutex<CurrentProcess>>
}

#[derive(Clone, Debug)]
struct CurrentProcess {
	name: String,
	pid: u32,
	handle: HANDLE
}

#[get("/")]
fn index() -> &'static str {
	"Hello World"
}

// Get process names and ids [{name:id}]
#[get("/processes")]
fn get_processes() -> Json<Vec<ProcMap>>{
	// let values = mem_look::lister::print_process_names();
	let values = print_process_names();

	return Json(values);
}

#[get("/process/open/<pid>")]
fn open_pid(pid: u32, current: &State<ProcessState>) -> Result<String,status::BadRequest<String>>{
	
	let mut c = current.current.lock().unwrap();
	let mut process_handle = HANDLE::NULL;
	// assert_eq!(process_handle,c.handle);
	match c.name.as_str(){
		"None" => {
			process_handle = open_process(pid);
			let proc = get_process_name(process_handle);
			c.name = proc;
			c.pid = pid;
			c.handle = process_handle;
			Ok(c.name.clone())

		},
		_ => Err(status::BadRequest(Some("A process is already open. Please close process with /process/close".to_string())))

	}




}

#[get("/process")]
fn curr_state(current: &State<ProcessState>) -> Result<String,status::BadRequest<String>>{
	let c = current.current.lock().unwrap();
	match c.name.as_str() {
		"None" => Err(status::BadRequest(Some("No process open. Please open process with /process/open. List with /processes. ".to_string()))),
		_ => Ok(format!("{} is currently open", c.name.clone()))

	}
	
}


#[get("/process/close")]
fn close_pid(current: &State<ProcessState>) -> Result<String,status::BadRequest<String>>{
	
	let mut c = current.current.lock().unwrap();
	match c.name.as_str() {
		"None" => Err(status::BadRequest(Some("No process open. Please open process with /process/open/<pid>. List with /processes. ".to_string()))),
		_ => {
				let name = c.name.clone();
				close_process(c.handle);

				c.name = "None".to_string();
				c.pid = 0;
				c.handle = HANDLE::NULL;

				Ok(format!("Closed process {}", name))
			}			
	}


}

#[get("/process/modules")]
fn get_modules(current: &State<ProcessState>) -> Result<Json<Vec<String>>,status::BadRequest<String>>{
	let mut c = current.current.lock().unwrap();
	match c.name.as_str() {
		"None" => Err(status::BadRequest(Some("No process open. Please open process with /process/open/<pid>. List with /processes. ".to_string()))),
		_ => {
			let modules = print_modules(c.handle);
			Ok(Json(modules))
		}
	}
}



// Select process [id]
// Need some type of event loop with the handle??
// #[post("/process")]

#[rocket::main]
pub async fn run(){
	let curr = CurrentProcess{name: "None".to_string(), pid: 0, handle: HANDLE::NULL };
	let result = rocket::build()
	.manage(ProcessState{current: Arc::new(Mutex::new(curr))})
	.mount("/", routes![index,get_processes,open_pid,close_pid,curr_state,get_modules]).launch().await;
	drop(result);
	// match result {
	// 	Ok(result) => Ok(String::from("Running")),
	// 	Err(e) => Err(e)
	// }

}
