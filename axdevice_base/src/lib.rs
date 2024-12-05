#![no_std]
#![feature(trait_alias)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

//! This crate provides basic traits and structures for emulated devices of ArceOS hypervisor.
//!
//! This crate contains:
//! [`BaseDeviceOps`] trait: The trait that all emulated devices must implement.
//! [`EmulatedDeviceConfig`] struct: Represents the configuration of an emulated device for a virtual machine.
//! [`EmuDeviceType`] enum: Enumeration representing the type of emulator devices.

extern crate alloc;

use alloc::{boxed::Box, string::String, vec::Vec};
use axaddrspace::{
    device::{AccessWidth, DeviceAddrRange, PortRange, SysRegAddrRange},
    GuestPhysAddrRange,
};
use axerrno::AxResult;
use cpumask::CpuMask;

// TODO: support vgicv2
// pub(crate) mod emu_vgicdv2;
mod emu_type;

// pub use emu_config_notuse::EmulatedDeviceConfig;
pub use emu_type::EmuDeviceType;

/// Represents the configuration of an emulated device for a virtual machine.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmulatedDeviceConfig {
    /// The name of the device
    pub name: String,
    /// The base IPA (Intermediate Physical Address) of the device.
    pub base_ipa: usize,
    /// The length of the device.
    pub length: usize,
    /// The IRQ (Interrupt Request) ID of the device.
    pub irq_id: usize,
    /// The type of emulated device.
    pub emu_type: usize,
    /// The config_list of the device
    pub cfg_list: Vec<usize>,
}

/// Represents the context of a device read/write operation.
pub struct DeviceRWContext {
    pub vcpu_id: usize,
}

impl DeviceRWContext {
    /// Creates a new `DeviceRWContext` with the specified vCPU ID.
    pub fn new(vcpu_id: usize) -> Self {
        Self { vcpu_id }
    }
}

/// [`BaseDeviceOps`] is the trait that all emulated devices must implement.
pub trait BaseDeviceOps<R: DeviceAddrRange> {
    /// Returns the type of the emulated device.
    fn emu_type(&self) -> EmuDeviceType;
    /// Returns the address range of the emulated device.
    fn address_range(&self) -> R;
    /// Handles a read operation on the emulated device.
    fn handle_read(
        &self,
        addr: R::Addr,
        width: AccessWidth,
        context: DeviceRWContext,
    ) -> AxResult<usize>;
    /// Handles a write operation on the emulated device.
    fn handle_write(
        &self,
        addr: R::Addr,
        width: AccessWidth,
        val: usize,
        context: DeviceRWContext,
    ) -> AxResult;
    /// Sets the interrupt injector for the emulated device.
    fn set_interrupt_injector(&mut self, injector: Box<InterruptInjector>);
}

// trait aliases are limited yet: https://github.com/rust-lang/rfcs/pull/3437
/// [`BaseMmioDeviceOps`] is the trait that all emulated MMIO devices must implement.
/// It is a trait alias of [`BaseDeviceOps`] with [`GuestPhysAddrRange`] as the address range.
pub trait BaseMmioDeviceOps = BaseDeviceOps<GuestPhysAddrRange>;
/// [`BaseSysRegDeviceOps`] is the trait that all emulated system register devices must implement.
/// It is a trait alias of [`BaseDeviceOps`] with [`SysRegAddrRange`] as the address range.
pub trait BaseSysRegDeviceOps = BaseDeviceOps<SysRegAddrRange>;
/// [`BasePortDeviceOps`] is the trait that all emulated port devices must implement.
/// It is a trait alias of [`BaseDeviceOps`] with [`PortRange`] as the address range.
pub trait BasePortDeviceOps = BaseDeviceOps<PortRange>;

/// The maximum number of vCPUs supported.
pub const MAX_VCPU_NUM: usize = 64;

/// A closure that injects an interrupt to the specified vCPUs.
pub type InterruptInjector = dyn FnMut(CpuMask<{ MAX_VCPU_NUM }>, usize) -> AxResult<()>;
