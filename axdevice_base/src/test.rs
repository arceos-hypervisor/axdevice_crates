use alloc::{sync::Arc, vec::Vec};
use alloc::vec;
use axaddrspace::{device::AccessWidth, GuestPhysAddr, GuestPhysAddrRange};
use axerrno::AxResult;

use crate::{map_device_of_type, BaseDeviceOps, EmuDeviceType};

const DEVICE_A_TEST_METHOD_ANSWER: usize = 42;

struct DeviceA;

impl BaseDeviceOps<GuestPhysAddrRange> for DeviceA {
    fn emu_type(&self) -> EmuDeviceType {
        EmuDeviceType::Dummy
    }

    fn address_range(&self) -> GuestPhysAddrRange {
        (0x1000..0x2000).try_into().unwrap()
    }

    fn handle_read(&self, addr: GuestPhysAddr, _width: AccessWidth) -> AxResult<usize> {
        Ok(addr.as_usize())
    }

    fn handle_write(&self, _addr: GuestPhysAddr, _width: AccessWidth, _val: usize) -> AxResult {
        Ok(())
    }
}

impl DeviceA {
    /// A test method unique to DeviceA.
    pub fn test_method(&self) -> usize {
        DEVICE_A_TEST_METHOD_ANSWER
    }
}

struct DeviceB;

impl BaseDeviceOps<GuestPhysAddrRange> for DeviceB {
    fn emu_type(&self) -> EmuDeviceType {
        EmuDeviceType::Dummy
    }

    fn address_range(&self) -> GuestPhysAddrRange {
        (0x2000..0x3000).try_into().unwrap()
    }

    fn handle_read(&self, addr: GuestPhysAddr, _width: AccessWidth) -> AxResult<usize> {
        Ok(addr.as_usize())
    }

    fn handle_write(&self, _addr: GuestPhysAddr, _width: AccessWidth, _val: usize) -> AxResult {
        Ok(())
    }
}

#[test]
fn test_device_type_test() {
    let devices: Vec<Arc<dyn BaseDeviceOps<GuestPhysAddrRange>>> = vec![
        Arc::new(DeviceA),
        Arc::new(DeviceB),
    ];

    let mut device_a_found = false;
    for device in devices {
        assert_eq!(device.handle_read(0x2000.into(), AccessWidth::Byte), Ok(0x2000));

        if let Some(answer) = map_device_of_type(&device, |d: &DeviceA| d.test_method()) {
            assert_eq!(answer, DEVICE_A_TEST_METHOD_ANSWER);
            device_a_found = true;
        }
    }
    assert!(device_a_found, "DeviceA was not found");
}
