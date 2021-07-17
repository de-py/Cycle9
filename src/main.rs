#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use bindings::Windows::Win32;
use Win32::System::ProcessStatus::{K32EnumProcesses,K32GetModuleFileNameExA,K32EnumProcessModules,K32GetModuleInformation,MODULEINFO};
use Win32::System::SystemServices::{BOOL,PSTR,HINSTANCE,HANDLE,MEMORY_BASIC_INFORMATION};
use Win32::System::Threading::{PROCESS_ALL_ACCESS,PROCESS_ACCESS_RIGHTS,OpenProcess,PROCESS_QUERY_INFORMATION,PROCESS_VM_READ,PROCESS_VM_WRITE,PROCESS_VM_OPERATION};
use Win32::System::Diagnostics::Debug::{WIN32_ERROR,GetLastError,ReadProcessMemory,WriteProcessMemory};
use Win32::System::WindowsProgramming::{CloseHandle};
use std::ffi::{c_void};
// use Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot,CREATE_TOOLHELP_SNAPSHOT_FLAGS,Module32First,MODULEENTRY32};
use Win32::System::Memory::{VirtualQueryEx};
use std::{mem, vec,convert};
use std::io::{self};
use std::convert::TryInto;
// use mem_look::read_memory::{open_process,get_memory_regions,scan_multiple_regions,get_module_info,Discovered,MemAddr,Scan};
use std::path::PathBuf;
use structopt::StructOpt;
use mem_server::rocket_srv::{run};
use futures::executor::block_on;
use mem_look::lister::{*};
use mem_look::read_memory::{*};
mod mem_server;
mod mem_look;


#[derive(StructOpt,Debug)]
#[structopt(name = "Memory Looker")]
struct Opt {
    // #[structopt(short="-p", long)]
    // pid: Option<u32>,

    // #[structopt(short="-n",long,required_if("list-processes","false"))]
    #[structopt(short="-n",long,help("Name of module in the process to scan"),required_unless_one(&["list-processes","run-server"]))]
    module_name: Option<String>,
    
    #[structopt(short,long,help("List all processes"))]
    list_processes:  bool,

    #[structopt(short,long,help("Process id of process to scan"),required_unless_one(&["list-processes","run-server"]))]
    process_id:  Option<u32>,

    #[structopt(short,long,help("Run Server"))]
    run_server:  bool,

    #[structopt(short,long,help("Process id of process to scan"),required_unless_one(&["list-processes","run-server",]),conflicts_with_all(&["uint","int","double"]))]
    float:  Option<f32>,

    #[structopt(short,long,help("Process id of process to scan"),required_unless_one(&["list-processes","run-server",]),conflicts_with_all(&["float","int","double"]))]
    uint:  Option<u32>,
   
    #[structopt(short,long,help("Process id of process to scan"),required_unless_one(&["list-processes","run-server",]),conflicts_with_all(&["uint","float","double"]))]
    int:  Option<i32>,

    #[structopt(short,long,help("Process id of process to scan"),required_unless_one(&["list-processes","run-server",]),conflicts_with_all(&["uint","int","float"]))]
    double:  Option<f64>

}



fn main() {
    // run();


    let mut t = Scan::Float(4.4);
    let opt = Opt::from_args();

    if opt.run_server{
        println!("Hit");
        run();
    }

    if opt.list_processes{

        let m = print_process_names();
        for l in m{
           println!("{} {}",l.pid,l.name);
        }
    }

    match opt.module_name{
        None => {
            // println!("No module name given");
        }, 
        Some(ref x) => println!("{:?}",x)
    }


    if opt.process_id.is_some() && opt.module_name.is_some(){

        // TODO put this somewhere else
        let valid_protections: Vec<u32> = vec![0x02,0x04,0x08];
        let is_first_scan = true;


        
        let users = "d";



        let search_val = if opt.float.is_some() {
            Scan::Float(opt.float.unwrap())
        }   else if opt.uint.is_some() {
            Scan::Uint32(opt.uint.unwrap())
        }   else if opt.int.is_some() {
            Scan::Int32(opt.int.unwrap())
        }   else if opt.double.is_some() {
            Scan::Double(opt.double.unwrap())
        } else {
            panic!("done")
        };


    
        let process_handle = open_process(opt.process_id.unwrap());
        let module_info = get_module_info(opt.module_name.unwrap(), process_handle);
        let mem_regions = get_memory_regions(process_handle, module_info, valid_protections);
        let results = scan_multiple_regions(process_handle, &mem_regions, &search_val, is_first_scan);

        for r in results{
            println!("{:?}",r);
        }


    }





}





fn write_memory(base_addr: *mut c_void, write_size: usize, process_handle: HANDLE, write_val: u32){
    
     
    let writer = &write_val as *const _ as *const c_void;
    
    let mut lpnumberofbyteswritten: usize = 0;
    unsafe{
        let res: BOOL = WriteProcessMemory(process_handle,base_addr,writer,write_size,&mut lpnumberofbyteswritten);
        let error: WIN32_ERROR = GetLastError(); 
        assert_ne!(BOOL(0),res,"{:?}",error);
    }

}

















