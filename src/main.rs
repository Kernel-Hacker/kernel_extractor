mod log_utils;
mod device_utils;

use crate::device_utils::has_boot_config;
pub use crate::log_utils::init_log;
use std::io::Write;
use std::path;
use std::fs;
use std::process::Command;
use std::process::Stdio;

fn main() {
    init_log();

    if !nix::unistd::getuid().is_root() {
        log::error!("Please run with ROOT permission!");
        return;
    }

    let path = "/sdcard/my_kernel";
    let outfile = "/sdcard/my_kernel.tgz";

    if path::Path::new(path).exists() {
        log::info!("Cache directory exist, cleaning...");
        fs::remove_dir_all(path).expect("Error");
    }
    fs::create_dir(path).expect("Cannot create cache directory");
    log::info!("Cache directory created at {}", path);

    let mut boots = Vec::<String>::new();

    if device_utils::is_ab() {
        log::info!("A/B partition is supported");
        let ab = device_utils::get_ab();
        match ab {
            None => {
                log::info!("Cannot get current slot, both of them will be extracted");
                boots.push("/dev/block/by-name/boot_a".to_string());
                boots.push("/dev/block/by-name/boot_b".to_string());
            },
            Some(true) => {
                log::info!("Current slot: a");
                boots.push("/dev/block/by-name/boot_a".to_string());
            },
            Some(false) => {
                log::info!("Current slot: b");
                boots.push("/dev/block/by-name/boot_b".to_string());
            }
        }
    }else {
        log::info!("A/B partition is not supported");
        boots.push("/dev/block/by-name/boot".to_string());
    }

    for i in boots {
        log::info!("Start to extract {}", i);
        if device_utils::extract_boot(&i, path.to_string()) {
            log::info!("Extract {} succeed", i);
        }else{
            log::error!("Extract {} failed", i);
        }
    }

    let props = device_utils::get_props();
    let mut f = fs::File::create(format!("{}/props.txt", path)).expect("Error");
    f.write(props.as_bytes()).expect("Error");
    log::info!("props extracted");

    let config = device_utils::get_kernel_config();
    let mut f = fs::File::create(format!("{}/config.txt", path)).expect("Error");
    f.write(config.as_bytes()).expect("Error");
    log::info!("/proc/config.gz decompressed and extracted");

    if device_utils::has_module_sig_check(&config) {
        log::warn!("Module signature check is found");
    }

    log::info!("Extracting kallsyms 0/2");
    let outputs = fs::File::create("/proc/sys/kernel/kptr_restrict").expect("Cannout open file");
    Command::new("echo").arg("0").stdout(Stdio::from(outputs)).output().expect("Error");
    log::info!("kptr_restrict 0");

    Command::new("cp").arg("/proc/kallsyms").arg(format!("{}/kallsyms1.txt", path)).output().expect("Error");
    log::info!("kallsyms 1/2 extracted");

    let outputs = fs::File::create("/proc/sys/kernel/kptr_restrict").expect("Cannout open file");
    Command::new("echo").arg("1").stdout(Stdio::from(outputs)).output().expect("Error");
    log::info!("kptr_restrict 1");

    Command::new("cp").arg("/proc/kallsyms").arg(format!("{}/kallsyms2.txt", path)).output().expect("Error");
    log::info!("kallsyms 2/2 extracted");
    
    let outputs = fs::File::create("/proc/sys/kernel/kptr_restrict").expect("Cannout open file");
    Command::new("echo").arg("2").stdout(Stdio::from(outputs)).output().expect("Error");
    log::info!("kptr_restrict restore to 2");

    log::info!("Extracting cmdline");
    Command::new("cp").arg("/proc/cmdline").arg(format!("{}/cmdline.txt", path)).output().expect("Error");
    log::info!("/proc/cmdline extracted");

    log::info!("Extracting bootconfig");
    if has_boot_config() {
        Command::new("cp").arg("/proc/bootconfig").arg(format!("{}/bootconfig.txt", path)).output().expect("Error");
        log::info!("/proc/bootconfig extracted");
    }else {
        log::warn!("/proc/bootconfig is not existed");
    }
    
    log::info!("Start to compress");
    Command::new("tar").arg("czvf").arg(outfile).arg(format!("{}/", path)).output().expect("Error");
    log::info!("Compressed");

    log::info!("Cleaning cache directory");
    fs::remove_dir_all(path).expect("Error");
    log::info!("Finished! save to :{}", outfile);
    log::info!("You can use the following command to pull it!");
    log::info!("adb pull {} .", outfile);
}
