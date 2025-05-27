use sysinfo::{System, RefreshKind, ProcessesToUpdate, ProcessRefreshKind};

pub struct MemoryInfo {
    system: System
}

impl MemoryInfo {
    pub fn new() -> Self {
        if !sysinfo::IS_SUPPORTED_SYSTEM {
            log::error!("https://docs.rs/sysinfo does not support your platform");
            std::process::exit(1);
        }

        Self {
            system: System::new_with_specifics(RefreshKind::nothing())
        }
    }

    /// memory used by this process in GB
    #[allow(clippy::cast_precision_loss)]
    pub fn used(&mut self) -> f32 {
        let pid = sysinfo::get_current_pid().unwrap();
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[pid]),
            true,
            ProcessRefreshKind::nothing().with_memory()
        );
        let process = self.system.process(pid).unwrap();
        process.memory() as f32 * 1e-9
    }

    /// memory available (including swap) on this system in GB
    #[allow(clippy::cast_precision_loss)]
    pub fn available(&mut self) -> f32 {
        self.system.refresh_memory();
        self.system.available_memory() as f32 * 1e-9
    }
}
