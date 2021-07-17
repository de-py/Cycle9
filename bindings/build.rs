fn main() {
    windows::build!(
        Windows::Win32::System::ProcessStatus::{K32EnumProcesses,K32GetModuleFileNameExA,K32EnumProcessModules,K32GetModuleInformation,MODULEINFO},
        Windows::Win32::System::SystemServices::{HANDLE,BOOL,PSTR,HINSTANCE,GetModuleHandleA,MEMORY_BASIC_INFORMATION},
        Windows::Win32::System::Threading::{GetProcessId,PROCESS_ACCESS_RIGHTS,OpenProcess,PROCESS_ALL_ACCESS,PROCESS_QUERY_INFORMATION,PROCESS_VM_READ,PROCESS_VM_WRITE,PROCESS_VM_OPERATION},
        Windows::Win32::System::Diagnostics::Debug::{WIN32_ERROR,GetLastError,ReadProcessMemory,WriteProcessMemory},
        Windows::Win32::System::WindowsProgramming::{CloseHandle},
        Windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot,CREATE_TOOLHELP_SNAPSHOT_FLAGS,Module32First},
        Windows::Win32::System::Memory::{VirtualQueryEx}
        

    );
}