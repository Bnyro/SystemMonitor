#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{thread, time::Duration};

use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, DiskExt, System, SystemExt};

#[derive(Serialize, Deserialize, Debug)]
struct SystemInfo {
    sys_name: String,
    sys_version: String,
    kernel_version: String,
    host_name: String,
    cpu_usage: f32,
    ram_usage: f32,
    disk_usage: f32,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_sys_info() -> String {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    sys.refresh_cpu();
    thread::sleep(Duration::from_millis(200));
    sys.refresh_cpu();

    let cpu_usage = avg(sys
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect::<Vec<f32>>())
        / 100 as f32;

    let ram_usage = sys.used_memory() as f32 / sys.total_memory() as f32;

    let disks = sys.disks().iter();
    let disk_space = disks.clone().map(|disk| disk.total_space()).sum::<u64>() as f32;
    let used_space = disks
        .map(|disk| disk.total_space() - disk.available_space())
        .sum::<u64>() as f32;
    let disk_usage = used_space / disk_space;

    let sys_info = SystemInfo {
        sys_name: sys.name().expect("Unknown system name"),
        sys_version: sys.os_version().expect("Unknown system version"),
        kernel_version: sys.kernel_version().expect("Unknown Kernel verion"),
        host_name: sys.host_name().expect("Unknown host name"),
        cpu_usage,
        ram_usage,
        disk_usage,
    };

    serde_json::to_string(&sys_info).unwrap()
}

fn avg(vec: Vec<f32>) -> f32 {
    vec.iter().sum::<f32>() / vec.len() as f32
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_sys_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
