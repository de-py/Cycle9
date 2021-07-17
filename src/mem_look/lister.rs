use bindings::Windows::Win32;
use Win32::System::ProcessStatus::{K32EnumProcesses,K32GetModuleFileNameExA,K32EnumProcessModules,K32GetModuleInformation,MODULEINFO};
use Win32::System::SystemServices::{BOOL,PSTR,HINSTANCE,HANDLE,MEMORY_BASIC_INFORMATION};
use Win32::System::Threading::{PROCESS_ALL_ACCESS,PROCESS_ACCESS_RIGHTS,OpenProcess,};
use Win32::System::Diagnostics::Debug::{WIN32_ERROR,GetLastError,ReadProcessMemory};
use Win32::System::WindowsProgramming::{CloseHandle};
use std::ffi::{c_void};
// use Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot,CREATE_TOOLHELP_SNAPSHOT_FLAGS,Module32First,MODULEENTRY32};
// use Win32::System::Memory::{VirtualQueryEx};
// use std::{mem, vec,convert};
use std::io::{self};
use std::convert::TryInto;
// use rocket::serde::json::Json;
// use rayon::prelude::*;
use rocket::serde::{Serialize, json::Json};




#[derive(Serialize)]
pub struct ProcMap{
    pub pid: u32,
    pub name: String,
}


pub fn print_process_names()->Vec<ProcMap>{
		let pids = get_process_ids();
		let mut names: Vec<ProcMap> = Vec::new();
	
		for pid in pids.iter(){
			let mut arr: Vec<u8> = vec![0;512];
			let process_handle = unsafe{ OpenProcess(PROCESS_ACCESS_RIGHTS(0x0400),BOOL(0),*pid)};
			let error: WIN32_ERROR = unsafe{GetLastError()};
	
			if process_handle != HANDLE::NULL{
				let lphmodule = HINSTANCE::default();
				let arr_buffer = PSTR(arr.as_mut_ptr());
				let res: u32 = unsafe{K32GetModuleFileNameExA(process_handle,lphmodule,arr_buffer,arr.len() as u32)};
				let error: WIN32_ERROR = unsafe{GetLastError()};
				assert_ne!(0,res,"{:?}",error);
				let process_name: String = String::from_utf8_lossy(&arr).trim_matches(char::from(0)).to_string();
				let short_name: Vec<&str> = process_name.split("\\").collect();
	
				if process_name.len() > 0{
					names.push(ProcMap{pid: *pid, name: short_name.iter().next_back().unwrap().to_string()});
					// names.insert(*pid, short_name.iter().next_back().unwrap().to_string());
					// println!("{} {}",pid, short_name.iter().next_back().unwrap());
				}
				unsafe{CloseHandle(process_handle);}
	
	
		
			}
	
	   
		}
	
		return names;
	
	
}

pub fn get_process_name(process_handle: HANDLE) -> String{
	let mut arr: Vec<u8> = vec![0;512];
	let error: WIN32_ERROR = unsafe{GetLastError()};
	let mut result = "None Found".to_string();
	
	if process_handle != HANDLE::NULL{
		let lphmodule = HINSTANCE::default();
		let arr_buffer = PSTR(arr.as_mut_ptr());
		let res: u32 = unsafe{K32GetModuleFileNameExA(process_handle,lphmodule,arr_buffer,arr.len() as u32)};
		let error: WIN32_ERROR = unsafe{GetLastError()};
		assert_ne!(0,res,"{:?}",error);
		let process_name: String = String::from_utf8_lossy(&arr).trim_matches(char::from(0)).to_string();
		let short_name: Vec<&str> = process_name.split("\\").collect();

		if process_name.len() > 0 {
			result = short_name.iter().next_back().unwrap().to_string();
		}
	}

	return result;
}
	
fn get_process_ids() -> Vec<u32>{
	let mut pids = vec![0;1024];
	let mut lpcbneeded: u32 = 0;


	unsafe{
		let res: BOOL = K32EnumProcesses(pids.as_mut_ptr(),4096,&mut lpcbneeded);
		let error: WIN32_ERROR = GetLastError();
		assert_ne!(BOOL(0),res,"{:?}",error);
	};

	pids.retain(|x| *x != 0);

	return pids;

}


pub fn print_modules(process_handle: HANDLE) -> Vec<String>{

	let mut lphmodule: Vec<HINSTANCE> = vec![HINSTANCE(1);512];
	let cb: u32 = lphmodule.len() as u32;
	let mut lpcbneeded: u32 = 0;
	let mut response: Vec<String> = Vec::new();
	unsafe{
		let result: BOOL = K32EnumProcessModules(process_handle,lphmodule.as_mut_ptr(),cb,&mut lpcbneeded);
		let error: WIN32_ERROR = GetLastError();

		assert_ne!(BOOL(0),result,"{:?}",error);
		
		lphmodule.truncate(lpcbneeded as usize);

		{
			for module in lphmodule.iter(){

				let mut arr: Vec<u8> = vec![0;512];
				let lpfilename: PSTR = PSTR(arr.as_mut_ptr());
				let nsize: u32 = arr.len() as u32;
				let res: u32 = K32GetModuleFileNameExA(process_handle, module, lpfilename, nsize);
				let error2: WIN32_ERROR = GetLastError();
				if res != 0 {
					let module_name: String = String::from_utf8_lossy(&arr).trim_matches(char::from(0)).to_string();
					let short_name: Vec<&str> = module_name.split("\\").collect();

					if module_name.len() > 0{
						response.push(short_name.iter().next_back().unwrap().to_string());
					}


				}

			}
		}

	
	}
	return response;

}