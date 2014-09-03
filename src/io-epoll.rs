
use native::io::file::{fd_t, FileDesc}
use libc::{ uint32_t, uint64_t };
use std::ptr;

use epoll::*;


struct IOEpoll {
  epollfd : fd_t;

  fn new() {
    let fd = epoll_create();
    IOEpoll { epollfd : fd }
  }

}

impl Drop for IOEpoll {
  // this makes me feel funny
  fn drop(&mut self) {
    let f = FileDesc(epollfd, true);
  }
}

fn calc_evt_mask(events: &[IOPollEVent) -> uint32_t {
  events.iter().fold(0, |a, &b| { 
      b | match a {
        Read => EPOLLIN,
        Write => EPOLLOUT,
        Hangup => EPOLLHUP | EPOLLRDHUP,
        Error => EPOLLERR
      }
    });
}

impl IOPoll for IOEpoll {
  // add a new file descriptor to the polling mechanism to be monitored
  // events is an array of what events should be monitored for the fd
  fn load(&self, subject: fd_t, events: &[IOPollEvent]) -> bool {
    let emask : uint32_t = calc_evt_mask(events);
    let data = epoll_data_t { data : subject };
    0 == epoll_ctl(self.epollfd, EPOLL_CTL_ADD, subject, Struct_epoll_event { events: emask, data: data })
  }

  // remove a file descriptor from the polling mechanism
  fn unload(&self, subject: fd_t) -> bool {
    let tmp = Struct_epoll_event { 0, epoll_data_t { data : [1] } };
    0 == epoll_ctl(self.epollfd, EPOLL_CTL_DEL, subject, &tmp)
  }

  // modify the settings of the polling mechanism
  fn modify(&self, subject: fd_t, events: &[IOPollEvent]) -> bool {
    let emask : uint32_t = calc_evt_mask(events);
    let data = epoll_data_t { data : subject };
    0 == epoll_ctl(self.epollfd, EPOLL_CTL_MOD, subject, Struct_epoll_event { events: emask, data: data })
  }

  // poll all of the supplied file descriptors for their defined events
  // timeout is how long to wait in milliseconds before giving up
  // subjects is a prepopulated array which will be filled with 
  // the filedescriptors which have events waiting
  // returns the number of events collected
  fn wait(&self, timeout: int, subjects: &mut[fd_t]) -> int {
  }
}
