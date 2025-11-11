// @MS 
use super::*;
use crate::phy::Medium;
use std::os::unix::io::{AsRawFd, RawFd};
#[derive(Debug)]
pub struct l4re_net {
    //protocol: libc::c_short,
    //lower: libc::c_int,
    //ifreq: ifreq,
}

impl AsRawFd for l4re_net {
    fn as_raw_fd(&self) -> RawFd {
        0 // @MS must be implemented if .as_rau_fd() is called
    }
}

impl Drop for l4re_net {
    fn drop(&mut self) {
        // implement drop if needed ( if cleanup is needed )
    }
}

impl l4re_net {
    pub fn new(name: &str, medium: Medium) -> io::Result<l4re_net> {
        Ok(l4re_net { })
    }

    pub fn interface_mtu(&mut self) -> io::Result<usize> {
        return Err(io::Error::last_os_error());
    }

    pub fn bind_interface(&mut self) -> io::Result<()> {
        return Err(io::Error::last_os_error());
    }

    pub fn recv(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
       return Err(io::Error::last_os_error());
    }

    pub fn send(&mut self, buffer: &[u8]) -> io::Result<usize> {
        return Err(io::Error::last_os_error());
    }
}
