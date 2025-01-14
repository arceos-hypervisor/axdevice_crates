#![no_std]

//! This crate provides basic traits and structures for emulated devices of ArceOS hypervisor.
//!
//! This crate contains:
//! [`BaseDeviceOps`] trait: The trait that all emulated devices must implement.
//! [`EmuDeviceType`] enum: Enumeration representing the type of emulator devices.

extern crate alloc;

use memory_addr::AddrRange;

use axaddrspace::GuestPhysAddr;
use axerrno::AxResult;

mod emu_type;
// pub use emu_config_notuse::EmulatedDeviceConfig;
pub use emu_type::EmuDeviceType;

/// [`BaseDeviceOps`] is the trait that all emulated devices must implement.
pub trait BaseDeviceOps {
    /// Returns the type of the emulated device.
    fn emu_type(&self) -> EmuDeviceType;
    /// Returns the address range of the emulated device.
    fn address_range(&self) -> AddrRange<GuestPhysAddr>;
    /// Handles a read operation on the emulated device.
    fn handle_read(&self, addr: GuestPhysAddr, width: usize, vcpu: &dyn VCpuIf) -> AxResult<usize>;
    /// Handles a write operation on the emulated device.
    fn handle_write(&self, addr: GuestPhysAddr, width: usize, val: usize, vcpu: &dyn VCpuIf);
}

/// A trait representing a virtual CPU interface.
///
/// This trait defines the basic operations that can be performed on a virtual CPU.
pub trait VCpuIf {
    /// Returns the ID of the virtual CPU.
    ///
    /// # Returns
    /// * `usize` - The ID of the virtual CPU.
    fn vcpu_id(&self) -> usize;

    /// Sets the value of a general-purpose register.
    ///
    /// # Arguments
    /// * `reg` - The index of the register to set.
    /// * `val` - The value to set the register to.
    fn set_gpr(&self, reg: usize, val: usize);
}
