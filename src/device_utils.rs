use std::fs;
use std::io::Read;
use std::path::Path;
use flate2::read::GzDecoder;
use std::process::Command;

pub fn is_ab() -> bool{
    Path::new("/dev/block/by-name/boot_a").exists()
}

pub fn has_boot_config() -> bool{
    Path::new("/proc/bootconfig").exists()
}

// true: A false: B
pub fn get_ab() -> Option<bool>{
    let output = Command::new("getprop")
        .arg("ro.boot.slot_suffix")
        .output().expect("Error");
    let str = String::from_utf8(output.stdout).expect("Error");
    if str.contains("_a") {
        return Some(true);
    }else if str.contains("_b") {
        return Some(false);
    }
    return None;
}

pub fn extract_boot(boot: &String, save_path: String) -> bool{
    let name = save_path + "/" + boot.split("/").last().unwrap() + ".img";
    let result = Command::new("dd")
        .arg(String::from("if=") + boot)
        .arg(String::from("of=") + &name)
        .output();
    if result.is_ok() {
        return true;
    }else{
        log::error!("{}", result.err().unwrap());
        return false;
    }
}

pub fn get_props() -> String{
    let output = Command::new("getprop").output().expect("Error");
    return String::from_utf8(output.stdout).expect("Error");
}

pub fn get_kernel_config() -> String{
    let mut f = fs::File::open("/proc/config.gz").expect("Error");
    let mut buf = Vec::<u8>::new();
    f.read_to_end(&mut buf).expect("Error");
    let mut d = GzDecoder::new(&buf[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    log::trace!("{}", s);
    s
}

pub fn has_module_sig_check(config: &String) -> bool{
    config.contains("CONFIG_MODULE_SIG=y")
}