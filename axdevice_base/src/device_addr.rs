use axaddrspace::GuestPhysAddr;
use memory_addr::AddrRange;

/// An address-like type that can be used to access devices.
pub trait DeviceAddr: Copy + Eq + Ord + core::fmt::Debug {}

/// A range of device addresses. It may be contiguous or not.
pub trait DeviceAddrRange {
    /// The address type of the range.
    type Addr: DeviceAddr;

    /// Returns whether the address range contains the given address.
    fn contains(&self, addr: Self::Addr) -> bool;
}

impl DeviceAddr for GuestPhysAddr {}

impl DeviceAddrRange for AddrRange<GuestPhysAddr> {
    type Addr = GuestPhysAddr;

    fn contains(&self, addr: Self::Addr) -> bool {
        Self::contains(*self, addr)
    }
}

/// A system register address.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct SysRegAddr(pub usize); // u32 seems to be enough, but we use usize for generality.

impl DeviceAddr for SysRegAddr {}

/// A range of system register addresses.
pub struct SysRegAddrRange {
    pub start: SysRegAddr,
    pub end: SysRegAddr,
}

impl SysRegAddrRange {
    /// Creates a new [`SysRegAddrRange`] instance.
    pub fn new(start: SysRegAddr, end: SysRegAddr) -> Self {
        Self { start, end }
    }
}

impl DeviceAddrRange for SysRegAddrRange {
    type Addr = SysRegAddr;

    fn contains(&self, addr: Self::Addr) -> bool {
        addr.0 >= self.start.0 && addr.0 < self.end.0
    }
}
