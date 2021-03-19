use indicatif::HumanBytes;
use log::info;
use sysinfo::{DiskExt, NetworkExt, ProcessorExt, SystemExt, UserExt};

pub use sysinfo::System as SystemInfo;

pub fn get_and_print_system_info() -> SystemInfo {
    let mut sys_info = sysinfo::System::new_all();
    sys_info.refresh_all();

    info!("CPU: {}", sys_info.get_global_processor_info().get_brand());
    info!("CPU cores (logical): {}", num_cpus::get());
    info!("CPU cores (physical): {}", num_cpus::get_physical());

    #[cfg(target_arch = "x86_64")]
    {
        info!(
            "{}: {}",
            "aes".to_uppercase(),
            std::is_x86_feature_detected!("aes")
        );
        info!(
            "{}: {}",
            "pclmulqdq".to_uppercase(),
            std::is_x86_feature_detected!("pclmulqdq")
        );
        info!(
            "{}: {}",
            "rdrand".to_uppercase(),
            std::is_x86_feature_detected!("rdrand")
        );
        info!(
            "{}: {}",
            "rdseed".to_uppercase(),
            std::is_x86_feature_detected!("rdseed")
        );
        info!(
            "{}: {}",
            "tsc".to_uppercase(),
            std::is_x86_feature_detected!("tsc")
        );
        info!(
            "{}: {}",
            "mmx".to_uppercase(),
            std::is_x86_feature_detected!("mmx")
        );
        info!(
            "{}: {}",
            "sse".to_uppercase(),
            std::is_x86_feature_detected!("sse")
        );
        info!(
            "{}: {}",
            "sse2".to_uppercase(),
            std::is_x86_feature_detected!("sse2")
        );
        info!(
            "{}: {}",
            "sse3".to_uppercase(),
            std::is_x86_feature_detected!("sse3")
        );
        info!(
            "{}: {}",
            "ssse3".to_uppercase(),
            std::is_x86_feature_detected!("ssse3")
        );
        info!(
            "{}: {}",
            "sse4.1".to_uppercase(),
            std::is_x86_feature_detected!("sse4.1")
        );
        info!(
            "{}: {}",
            "sse4.2".to_uppercase(),
            std::is_x86_feature_detected!("sse4.2")
        );
        info!(
            "{}: {}",
            "sse4a".to_uppercase(),
            std::is_x86_feature_detected!("sse4a")
        );
        info!(
            "{}: {}",
            "sha".to_uppercase(),
            std::is_x86_feature_detected!("sha")
        );
        info!(
            "{}: {}",
            "avx".to_uppercase(),
            std::is_x86_feature_detected!("avx")
        );
        info!(
            "{}: {}",
            "avx2".to_uppercase(),
            std::is_x86_feature_detected!("avx2")
        );
        info!(
            "{}: {}",
            "avx512f".to_uppercase(),
            std::is_x86_feature_detected!("avx512f")
        );
        info!(
            "{}: {}",
            "avx512cd".to_uppercase(),
            std::is_x86_feature_detected!("avx512cd")
        );
        info!(
            "{}: {}",
            "avx512er".to_uppercase(),
            std::is_x86_feature_detected!("avx512er")
        );
        info!(
            "{}: {}",
            "avx512pf".to_uppercase(),
            std::is_x86_feature_detected!("avx512pf")
        );
        info!(
            "{}: {}",
            "avx512bw".to_uppercase(),
            std::is_x86_feature_detected!("avx512bw")
        );
        info!(
            "{}: {}",
            "avx512dq".to_uppercase(),
            std::is_x86_feature_detected!("avx512dq")
        );
        info!(
            "{}: {}",
            "avx512vl".to_uppercase(),
            std::is_x86_feature_detected!("avx512vl")
        );
        info!(
            "{}: {}",
            "avx512ifma".to_uppercase(),
            std::is_x86_feature_detected!("avx512ifma")
        );
        info!(
            "{}: {}",
            "avx512vbmi".to_uppercase(),
            std::is_x86_feature_detected!("avx512vbmi")
        );
        info!(
            "{}: {}",
            "avx512vpopcntdq".to_uppercase(),
            std::is_x86_feature_detected!("avx512vpopcntdq")
        );
        info!(
            "{}: {}",
            "avx512vbmi2".to_uppercase(),
            std::is_x86_feature_detected!("avx512vbmi2")
        );
        info!(
            "{}: {}",
            "avx512gfni".to_uppercase(),
            std::is_x86_feature_detected!("avx512gfni")
        );
        info!(
            "{}: {}",
            "avx512vaes".to_uppercase(),
            std::is_x86_feature_detected!("avx512vaes")
        );
        info!(
            "{}: {}",
            "avx512vpclmulqdq".to_uppercase(),
            std::is_x86_feature_detected!("avx512vpclmulqdq")
        );
        info!(
            "{}: {}",
            "avx512vnni".to_uppercase(),
            std::is_x86_feature_detected!("avx512vnni")
        );
        info!(
            "{}: {}",
            "avx512bitalg".to_uppercase(),
            std::is_x86_feature_detected!("avx512bitalg")
        );
        info!(
            "{}: {}",
            "avx512bf16".to_uppercase(),
            std::is_x86_feature_detected!("avx512bf16")
        );
        info!(
            "{}: {}",
            "avx512vp2intersect".to_uppercase(),
            std::is_x86_feature_detected!("avx512vp2intersect")
        );
        info!(
            "{}: {}",
            "f16c".to_uppercase(),
            std::is_x86_feature_detected!("f16c")
        );
        info!(
            "{}: {}",
            "fma".to_uppercase(),
            std::is_x86_feature_detected!("fma")
        );
        info!(
            "{}: {}",
            "bmi1".to_uppercase(),
            std::is_x86_feature_detected!("bmi1")
        );
        info!(
            "{}: {}",
            "bmi2".to_uppercase(),
            std::is_x86_feature_detected!("bmi2")
        );
        info!(
            "{}: {}",
            "abm".to_uppercase(),
            std::is_x86_feature_detected!("abm")
        );
        info!(
            "{}: {}",
            "lzcnt".to_uppercase(),
            std::is_x86_feature_detected!("lzcnt")
        );
        info!(
            "{}: {}",
            "tbm".to_uppercase(),
            std::is_x86_feature_detected!("tbm")
        );
        info!(
            "{}: {}",
            "popcnt".to_uppercase(),
            std::is_x86_feature_detected!("popcnt")
        );
        info!(
            "{}: {}",
            "fxsr".to_uppercase(),
            std::is_x86_feature_detected!("fxsr")
        );
        info!(
            "{}: {}",
            "xsave".to_uppercase(),
            std::is_x86_feature_detected!("xsave")
        );
        info!(
            "{}: {}",
            "xsaveopt".to_uppercase(),
            std::is_x86_feature_detected!("xsaveopt")
        );
        info!("{}: {}", "xsaves", std::is_x86_feature_detected!("xsaves"));
        info!("{}: {}", "xsavec", std::is_x86_feature_detected!("xsavec"));
        info!(
            "{}: {}",
            "cmpxchg16b".to_uppercase(),
            std::is_x86_feature_detected!("cmpxchg16b")
        );
        info!(
            "{}: {}",
            "adx".to_uppercase(),
            std::is_x86_feature_detected!("adx")
        );
        info!(
            "{}: {}",
            "rtm".to_uppercase(),
            std::is_x86_feature_detected!("rtm")
        );
    }

    for component in sys_info.get_components() {
        info!("{:?}", component);
    }

    for disk in sys_info.get_disks() {
        info!(
            "Disk: {:?}, Type: {:?}, FS: {}, {} / {}",
            disk.get_name(),
            disk.get_type(),
            String::from_utf8_lossy(disk.get_file_system()),
            HumanBytes(disk.get_total_space() - disk.get_available_space()),
            HumanBytes(disk.get_total_space())
        );
    }

    info!(
        "Total memory: {} GB",
        HumanBytes(sys_info.get_total_memory())
    );
    info!("Used memory: {} GB", HumanBytes(sys_info.get_used_memory()));
    info!("Total swap: {} GB", HumanBytes(sys_info.get_total_swap()));
    info!("Used swap: {} GB", HumanBytes(sys_info.get_used_swap()));

    info!(
        "System name: {}",
        sys_info
            .get_name()
            .unwrap_or_else(|| String::from("Unknown"))
    );
    info!(
        "System kernel version: {}",
        sys_info
            .get_kernel_version()
            .unwrap_or_else(|| String::from("Unknown"))
    );
    info!(
        "System OS version: {}",
        sys_info
            .get_os_version()
            .unwrap_or_else(|| String::from("Unknown"))
    );
    info!(
        "Machine name: {}",
        sys_info
            .get_host_name()
            .unwrap_or_else(|| String::from("Unknown"))
    );

    for user in sys_info.get_users() {
        info!(
            "User {} is in {} groups",
            user.get_name(),
            user.get_groups().len()
        );
    }

    for (interface_name, data) in sys_info.get_networks() {
        info!(
            "[{}] in: {}, out: {}",
            interface_name,
            data.get_received(),
            data.get_transmitted(),
        );
    }

    sys_info
}
