use memory_addr::PhysAddr;

#[crate_interface::def_interface]
pub trait MemoryApi {
    fn alloc_frame() -> Option<PhysAddr>;
    fn dealloc_frame(addr: PhysAddr);
}

#[crate_interface::def_interface]
pub trait VMVCpuApi {
    fn set_gpr(gpr: u8, value: u64);
    fn get_gpr(gpr: u8) -> u64;
}

#[crate_interface::def_interface]
pub trait TimerApi {
    fn register_timer();
}