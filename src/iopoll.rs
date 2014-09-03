/* abstract interface for platform agnosting async IO polling */

use native::io::file::{fd_t}

enum IOPollEvent {
  Read,
  Write, 
  Hangup,
  Error
}

trait IOPoll {
  // add a new file descriptor to the polling mechanism to be monitored
  // events is an array of what events should be monitored for the fd
  fn load(&self, subject: fd_t, events: &[IOPollEvent]),

  // remove a file descriptor from the polling mechanism
  fn unload(&self, subject: fd_t),

  // modify the settings of the polling mechanism
  fn modify(&self, subject: fd_t, events: &[IOPollEvent]),

  // poll all of the supplied file descriptors for their defined events
  // timeout is how long to wait in milliseconds before giving up
  // subjects is a prepopulated array which will be filled with 
  // the filedescriptors which have events waiting
  fn wait(&self, timeout: int, subjects: &mut[fd_t])
}
