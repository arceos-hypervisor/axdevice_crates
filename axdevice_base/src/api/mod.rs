mod macro_def;
use alloc::boxed::Box;
use memory_addr::PhysAddr;
use macro_def::def_device_api;

pub use paste::paste as __paste;

/// Memory management API
pub mod memory {
    use memory_addr::PhysAddr;

    super::def_device_api! {
        memory {
            /// Allocate a frame
            fn alloc_frame() -> Option<PhysAddr>;
            /// Deallocate a frame
            fn dealloc_frame(addr: PhysAddr);
        }
    }
}

/// Timer API
pub mod timer {
    use core::time::Duration;
    use alloc::boxed::Box;

    pub type TimeValue = Duration;
    pub type CancelToken = usize;

    super::def_device_api! {
        timer {
            fn current_time() -> TimeValue;
            /// Register a timer
            fn register_timer(deadline: TimeValue, callback: Box<dyn FnOnce(TimeValue) + Send + 'static>) -> CancelToken;
            /// Cancel a timer
            fn cancel_timer(token: CancelToken);
            /// Convert cycles to time
            fn ticks_to_time(cycles: u64) -> TimeValue;
            /// Convert time to cycles
            fn time_to_ticks(time: TimeValue) -> u64;
        }
    }
}
