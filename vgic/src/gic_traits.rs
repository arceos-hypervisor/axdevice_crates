use crate_interface::call_interface;

#[crate_interface::def_interface]
pub trait GicTrait {
    fn set_enable(vector: usize, enable: bool);
    fn get_enable(vector: usize) -> bool;
    fn get_typer() -> u32;
    fn get_iidr() -> u32;

    fn set_state(int_id: usize, state: usize, current_cpu_id: usize);
    fn get_state(int_id: usize) -> usize;

    fn set_icfgr(int_id: usize, cfg: u8);

    fn get_target_cpu(int_id: usize) -> usize;
    fn set_target_cpu(int_id: usize, target: u8);

    fn get_priority(int_id: usize) -> usize;
    fn set_priority(int_id: usize, priority: u8);
}

pub struct GicInterface;

/// Implementation of [`GicTrait`] by crate_interface::call_interface,
/// to provide an easy-to-use interface to
/// the [vgic] crate.
impl GicInterface {
    pub fn set_enable(vector: usize, enable: bool) {
        call_interface!(GicTrait::set_enable(vector, enable));
    }
    pub fn get_enable(vector: usize) -> bool {
        call_interface!(GicTrait::get_enable(vector))
    }

    pub fn get_typer() -> u32 {
        call_interface!(GicTrait::get_typer())
    }
    fn get_iidr() -> u32 {
        call_interface!(GicTrait::get_iidr())
    }

    pub fn set_state(int_id: usize, state: usize, current_cpu_id: usize) {
        call_interface!(GicTrait::set_state(int_id, state, current_cpu_id));
    }
    fn get_state(int_id: usize) -> usize {
        call_interface!(GicTrait::get_state(int_id))
    }

    fn set_icfgr(int_id: usize, cfg: u8) {
        call_interface!(GicTrait::set_icfgr(int_id, cfg));
    }

    fn get_target_cpu(int_id: usize) -> usize {
        call_interface!(GicTrait::get_target_cpu(int_id))
    }
    fn set_target_cpu(int_id: usize, target: u8) {
        call_interface!(GicTrait::set_target_cpu(int_id, target));
    }

    fn get_priority(int_id: usize) -> usize {
        call_interface!(GicTrait::get_priority(int_id))
    }
    fn set_priority(int_id: usize, priority: u8) {
        call_interface!(GicTrait::set_priority(int_id, priority));
    }
}
