mod constants {
    pub mod task_scheduler {
        /// Pattern of the `TaskScheduler::get` function.
        pub static GET_TASK_SCHEDULER: &str = "55 8B EC 64 A1 00 00 00 00 6A FF 68 ? ? ? ? 50 64 89 25 00 00 00 00 83 EC 14 64 A1 2C 00 00 00 8B 08 A1 ? ? ? ? 3B 81 08 00 00 00 7F 29 A1 ? ? ? ? 8B 4D F4 64 89 0D 00 00 00 00 8B E5 5D C3 8D 4D E4 E8 ? ? ? ? 68 ? ? ? ? 8D 45 E4 50 E8 ? ? ? ? 68 ? ? ? ? E8 ? ? ? ? 83 C4 04 83 3D ? ? ? ? ? 75 C1 68";

        /// Definition of the `TaskScheduler::get` function.
        pub type GetTaskScheduler = unsafe extern "cdecl" fn() -> *const usize;

        /// Offset of the `TaskScheduler` containing a `TaskScheduler::Job` of the first job.
        pub static JOB: isize = 0x134;

        /// Offset of the `TaskScheduler` containing a `TaskScheduler::Job` of the last job.
        pub static END: isize = 0x138;

        /// Offset of the `TaskScheduler::Job` containing a `std::string` of the job name.
        pub static NAME: isize = 0x10;
    }
}

pub mod task_scheduler;
