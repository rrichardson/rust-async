/* automatically generated by rust-bindgen 
cmd: bindgen -match epoll.h -o epoll.rs /usr/include/x86_64-linux-gnu/sys/epoll.h -I .
*/
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
use libc::{ uint32_t, uint64_t };

pub static EPOLL_CTL_ADD : ::libc::c_int = 1	/* Add a file decriptor to the interface.  */
pub static EPOLL_CTL_DEL : ::libc::c_int = 2	/* Remove a file decriptor from the interface.  */
pub static EPOLL_CTL_MOD : ::libc::c_int = 3	/* Change file decriptor epoll_event structure.  */

pub static EPOLL_CLOEXEC: ::libc::c_uint = 524288;
pub type Enum_EPOLL_EVENTS = ::libc::c_uint;
pub static EPOLLIN: ::libc::c_uint = 1;
pub static EPOLLPRI: ::libc::c_uint = 2;
pub static EPOLLOUT: ::libc::c_uint = 4;
pub static EPOLLRDNORM: ::libc::c_uint = 64;
pub static EPOLLRDBAND: ::libc::c_uint = 128;
pub static EPOLLWRNORM: ::libc::c_uint = 256;
pub static EPOLLWRBAND: ::libc::c_uint = 512;
pub static EPOLLMSG: ::libc::c_uint = 1024;
pub static EPOLLERR: ::libc::c_uint = 8;
pub static EPOLLHUP: ::libc::c_uint = 16;
pub static EPOLLRDHUP: ::libc::c_uint = 8192;
pub static EPOLLWAKEUP: ::libc::c_uint = 536870912;
pub static EPOLLONESHOT: ::libc::c_uint = 1073741824;
pub static EPOLLET: ::libc::c_uint = -2147483648;
#[repr(C)]
pub struct Union_epoll_data {
    pub data: [u64, ..1u],
}
impl Union_epoll_data {
    pub fn ptr(&mut self) -> *mut *mut ::libc::c_void {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn fd(&mut self) -> *mut ::libc::c_int {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn u32(&mut self) -> *mut uint32_t {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn u64(&mut self) -> *mut uint64_t {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub type epoll_data_t = Union_epoll_data;
#[repr(C)]
pub struct Struct_epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
extern "C" {
    pub fn epoll_create(__size: ::libc::c_int) -> ::libc::c_int;
    pub fn epoll_create1(__flags: ::libc::c_int) -> ::libc::c_int;
    pub fn epoll_ctl(__epfd: ::libc::c_int, __op: ::libc::c_int,
                     __fd: ::libc::c_int, __event: *mut Struct_epoll_event) ->
     ::libc::c_int;
    pub fn epoll_wait(__epfd: ::libc::c_int,
                      __events: *mut Struct_epoll_event,
                      __maxevents: ::libc::c_int, __timeout: ::libc::c_int) ->
     ::libc::c_int;

//  TODO make this work and provide a signal safe epoll API
//    pub fn epoll_pwait(__epfd: ::libc::c_int,
//                       __events: *mut Struct_epoll_event,
//                       __maxevents: ::libc::c_int, __timeout: ::libc::c_int,
//                       __ss: *const __sigset_t) -> ::libc::c_int;
}
