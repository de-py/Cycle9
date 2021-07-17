use bindings::Windows::Win32;
use Win32::System::ProcessStatus::{K32EnumProcesses,K32GetModuleFileNameExA,K32EnumProcessModules,K32GetModuleInformation,MODULEINFO};
use Win32::System::SystemServices::{BOOL,PSTR,HINSTANCE,HANDLE,MEMORY_BASIC_INFORMATION};
use Win32::System::Threading::{PROCESS_ALL_ACCESS,PROCESS_ACCESS_RIGHTS,OpenProcess,PROCESS_QUERY_INFORMATION,PROCESS_VM_READ,PROCESS_VM_WRITE,PROCESS_VM_OPERATION};
use Win32::System::Diagnostics::Debug::{WIN32_ERROR,GetLastError,ReadProcessMemory};
use Win32::System::WindowsProgramming::{CloseHandle};
use std::ffi::{c_void};
// use Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot,CREATE_TOOLHELP_SNAPSHOT_FLAGS,Module32First,MODULEENTRY32};
use Win32::System::Memory::{VirtualQueryEx};
use std::{mem, vec,convert};
use std::io::{self};
use std::convert::TryInto;
// use rocket::serde::json::Json;
// use rayon::prelude::*;
// use rocket::serde::{Serialize, json::Json};



pub struct Discovered {
    // val: [u8],
    count: u32,
}



#[derive(Copy, Clone, Debug)]
pub struct MemAddr {
    pub base: *mut c_void,
    size: usize
}




#[derive(Debug)]
pub enum Scan {
    Int32(i32),
    Uint32(u32),
    Float(f32),
    Double(f64)
}

impl Scan{
    pub fn len(&self) -> u32{
        match self{
            Scan::Int32(_) => 4,
            Scan::Uint32(_) => 4,
            Scan::Float(_) => 4,
            Scan::Double(_) => 8

        }

    }

    pub fn bytes(&self) -> Vec<u8>{
        match self{
            Scan::Uint32(value) => value.to_le_bytes().to_vec(),
            Scan::Int32(value) => value.to_le_bytes().to_vec(),
            Scan::Float(value) => value.to_le_bytes().to_vec(),
            Scan::Double(value) => value.to_le_bytes().to_vec()
        }
    }

    
}


pub fn open_process(process_id: u32) -> HANDLE {
    let process_handle: HANDLE = unsafe{OpenProcess(PROCESS_QUERY_INFORMATION|PROCESS_VM_READ|PROCESS_VM_WRITE|PROCESS_VM_OPERATION,BOOL(0),process_id)};
    let error: WIN32_ERROR = unsafe{GetLastError()};
    assert_ne!(HANDLE::NULL,process_handle, "{:?}",error);
    return process_handle;
}

pub fn close_process(process_handle: HANDLE){
    unsafe{CloseHandle(process_handle);}
}


pub fn read_memory(base_addr: *mut c_void, scan_size: usize, process_handle: HANDLE) ->Vec<u8>{
    
    let mut lpbuffer: Vec<u8> = vec![0;scan_size];
    
    let reader = lpbuffer.as_mut_ptr() as *mut _ as *mut c_void;
    
    let mut lpnumberofbytesread: usize = 0;
    unsafe{
        let res: BOOL = ReadProcessMemory(process_handle,base_addr,reader,scan_size,&mut lpnumberofbytesread);
        let error: WIN32_ERROR = GetLastError(); 
        assert_ne!(BOOL(0),res,"{:?}",error);
    }
    return lpbuffer;

}

pub fn get_memory_regions(process_handle: HANDLE, mod_info: MODULEINFO, valid_protections: Vec<u32>)->Vec<MemAddr>{

    let mut mem_info: MEMORY_BASIC_INFORMATION = MEMORY_BASIC_INFORMATION::default();
    let dwlength = mem::size_of::<MEMORY_BASIC_INFORMATION>();
    let mut base = mem_info.BaseAddress;
    let mut mem_regions: Vec<MemAddr> = Vec::new();
   

    unsafe{
        
        loop {
            let res = VirtualQueryEx(process_handle, base, &mut mem_info, dwlength);
            if res == 0 {
                break;
            }
            if valid_protections.contains(&mem_info.Protect){
                let mem_struct = MemAddr{base: mem_info.BaseAddress, size: mem_info.RegionSize };
                mem_regions.push(mem_struct);
            }

            base = mem_info.BaseAddress.add(mem_info.RegionSize);

        }

    }

    let error = unsafe{GetLastError()};
    assert_ne!(mem_regions.len(),0,"{:?}",error);

    return mem_regions;

}

pub fn get_module_info(user_module_name: String, process_handle: HANDLE) -> MODULEINFO {
    let mut lphmodule: Vec<HINSTANCE> = vec![HINSTANCE(1);512];
    let cb: u32 = lphmodule.len() as u32;
    let mut lpcbneeded: u32 = 0;
    let mut mod_info: MODULEINFO = MODULEINFO::default();
    
    unsafe{
        let result: BOOL = K32EnumProcessModules(process_handle,lphmodule.as_mut_ptr(),cb,&mut lpcbneeded);
        let error: WIN32_ERROR = GetLastError();
        assert_ne!(BOOL(0),result,"{:?}",error);
        lphmodule.truncate(lpcbneeded as usize);

        {
            for module in lphmodule.iter(){

                let mut arr: Vec<u8> = vec![0;512];
                // let mut lpfilename: PSTR = PSTR(arr.as_mut_ptr());
                let lpfilename: PSTR = PSTR(arr.as_mut_ptr());
                let nsize: u32 = arr.len() as u32;
                let res: u32 = K32GetModuleFileNameExA(process_handle, module, lpfilename, nsize);
                let error: WIN32_ERROR = GetLastError();
                
                assert_ne!(0,res,"{:?}",error);
                
                let module_name: String = String::from_utf8_lossy(&arr).trim_matches(char::from(0)).to_string();
                let name_split: Vec<&str> = module_name.split("\\").collect();
                let short_name = name_split.iter().next_back().unwrap().to_string();
                
                if short_name == user_module_name{

                    let cb: u32 = mem::size_of::<MODULEINFO>() as u32;
                    let bool_res: BOOL = K32GetModuleInformation(process_handle, module, &mut mod_info, cb);
                    let error: WIN32_ERROR = GetLastError();
                    
                    assert_ne!(BOOL(0),bool_res,"{:?}",error);
                    
                    break;
                }

            }
        }
        return mod_info;

    }

    
}


pub fn scan_multiple_regions(process_handle: HANDLE, regions: &Vec<MemAddr>, search_val: &Scan, first_scan: bool) -> Vec<MemAddr>{
    
    let mut results: Vec<MemAddr> = Vec::new();

    for m in regions.iter() {
        
        let read_region: Vec<u8> = read_memory(m.base,m.size, process_handle);
        let search_size: u32 = search_val.len();
        let found_vals: Vec<Discovered> = scan_single_region(&read_region, search_val);
        for found in found_vals{
             
            unsafe{
                if first_scan {
                    println!("{:?}",m.base);
                    let save_base = m.base.add((found.count*search_size) as usize);
                    let ma = MemAddr{ base: save_base, size: search_size as usize} ;
                    results.push(ma);
                }
                else{
                    results.push(*m);
                    
                }

            }
        }
    }
    
    return results;


}


fn scan_single_region(region_vector: &Vec<u8>,search_val: &Scan)-> Vec<Discovered>{
    let iter = region_vector.chunks(search_val.len() as usize);
    let mut counter = 0;
    let mut found: Vec<Discovered> = Vec::new();
    for it in iter{

        if it ==  &search_val.bytes()[..]{
            let d = Discovered { count: counter};
            found.push(d);
        }
            
        counter += 1;
    }

    return found;
    

}



