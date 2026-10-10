use sysinfo::{
    Components, System
};

pub fn retrieve_sys_information() -> (u64, f32) {

    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory_avaliable = ( sys.total_memory() / sys.used_memory() ) * 100;
    let CPU_use = sys.global_cpu_usage();

    (total_memory_avaliable, CPU_use)

}
