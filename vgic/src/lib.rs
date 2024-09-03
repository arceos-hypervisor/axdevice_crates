#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use axerrno::AxResult;

mod devops_impl;

mod gic_traits;

pub use gic_traits::GicInterface;
pub use gic_traits::GicTrait;

const PPI_ID_MAX: usize = 32; /* 16...31 */
const SPI_ID_MAX: usize = 512;
const GICD_LR_NUM: usize = 4;

/* ============ handler use offset ============= */
pub const VGICD_CTLR: usize = 0x0;
pub const VGICD_ISENABLER: usize = 0x2;
pub const VGICD_ICENABLER: usize = 0x3;
pub const VGICD_ISPENDR: usize = 0x4;
pub const VGICD_ICPENDR: usize = 0x5;
pub const VGICD_ISACTIVER: usize = 0x6;
pub const VGICD_ICACTIVER: usize = 0x7;
pub const VGICD_ICFGR: usize = 0x18;
pub const VGICD_SGIR: usize = 0x1e;

struct Vgicc {
    id: u32,
    pending_lr: [u32; SPI_ID_MAX],
    saved_lr: [u32; GICD_LR_NUM],

    saved_elsr0: u32,
    saved_apr: u32,
    saved_hcr: u32,

    isenabler: u32, // 0..31
    priorityr: [u8; PPI_ID_MAX],
}

pub struct Vgic {
    gicc: Vec<Vgicc>,

    ctrlr: u32,
    typer: u32,
    iidr: u32,

    gicd_igroupr: [u32; SPI_ID_MAX / 32],
    gicd_isenabler: [u32; SPI_ID_MAX / 32],
    gicd_ipriorityr: [u8; SPI_ID_MAX],
    gicd_itargetsr: [u8; SPI_ID_MAX],
    gicd_icfgr: [u32; SPI_ID_MAX / 16],
}



impl Vgic {
    pub fn new() -> Vgic {
        Vgic {
            gicc: Vec::new(),
            ctrlr: 0,
            typer: 0,
            iidr: 0,
            gicd_igroupr: [0; SPI_ID_MAX / 32],
            gicd_isenabler: [0; SPI_ID_MAX / 32],
            gicd_ipriorityr: [0; SPI_ID_MAX],
            gicd_itargetsr: [0; SPI_ID_MAX],
            gicd_icfgr: [0; SPI_ID_MAX / 16],
        }
    }

    fn handle_read8(&self, addr: usize) -> AxResult<usize> {
        Ok(0)
    }

    fn handle_read16(&self, addr: usize) -> AxResult<usize> {
        Ok(0)
    }

    fn handle_read32(&self, addr: usize) -> AxResult<usize> {
        Ok(0)
    }

    fn handle_write8(&self, addr: usize, val: usize) {}

    fn handle_write16(&self, addr: usize, val: usize) {}

    fn handle_write32(&self, addr: usize, val: usize) {}
}
