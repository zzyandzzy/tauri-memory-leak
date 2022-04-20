// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

#[tauri::command]
fn test_memory_leak_command() -> Result<Vec<u8>, String> {
    let v = vec![0xff; 1024 * 1024];
    Ok(v)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_memory_leak_command])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
