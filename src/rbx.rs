mod constants {
    /// `DataModel`
    pub mod data_model {
        /// Offset of `Render` containing a `DataModel` pointer.
        pub static OFFSET: isize = 0x28;
    }

    // `Instance`
    pub mod instance {}

    /// `ScriptContext`
    pub mod script_context {
        /// Offset of `WaitingHybridScriptsJob` containing a `ScriptContext` pointer.
        pub static OFFSET: isize = 0x130;
    }

    /// `TaskScheduler`
    pub mod task_scheduler {
        /// Pattern of the `TaskScheduler::get` function.
        pub static GET_TASK_SCHEDULER: &str = "55 8B EC 64 A1 00 00 00 00 6A FF 68 ? ? ? ? 50 64 89 25 00 00 00 00 83 EC 14 64 A1 2C 00 00 00 8B 08 A1 ? ? ? ? 3B 81 08 00 00 00 7F 29 A1 ? ? ? ? 8B 4D F4 64 89 0D 00 00 00 00 8B E5 5D C3 8D 4D E4 E8 ? ? ? ? 68 ? ? ? ? 8D 45 E4 50 E8 ? ? ? ? 68 ? ? ? ? E8 ? ? ? ? 83 C4 04 83 3D ? ? ? ? ? 75 C1 68";

        /// Offset of `TaskScheduler` containing a `TaskScheduler::Job` array of pointers.
        pub static JOBS: isize = 0x134;
    }
}

pub mod data_model;
pub mod instance;
pub mod script_context;
pub mod task_scheduler;
