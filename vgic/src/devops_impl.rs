use axdevice_base::BaseDeviceOps;
use axdevice_base::EmuDeviceType;


use axaddrspace::GuestPhysAddr;
use axerrno::AxResult;
use memory_addr::AddrRange;

use crate::Vgic;

impl BaseDeviceOps for Vgic {
    fn emu_type(&self) -> EmuDeviceType {
        EmuDeviceType::EmuDeviceTGicdV2
    }
    fn address_range(&self) -> AddrRange<GuestPhysAddr> {
        AddrRange::new(0x800_0000.into(), (0x800_0000 + 0x10000).into())
    }
    fn handle_read(&self, addr: GuestPhysAddr, width: usize) -> AxResult<usize> {
        let addr = addr.as_usize() & 0xfff;
        match width {
            0 => {
                return self.handle_read8(addr);
            }
            1 => {
                return self.handle_read16(addr);
            }
            2 => {
                return self.handle_read32(addr);
            }
            _ => Ok(0),
        }
    }
    fn handle_write(&self, addr: GuestPhysAddr, width: usize, val: usize) {
        let addr = addr.as_usize() & 0xfff;
        match width {
            0 => {
                self.handle_write8(addr, val);
            }
            1 => {
                self.handle_write16(addr, val);
            }
            2 => {
                self.handle_write32(addr, val);
            }
            _ => {}
        }
    }
}