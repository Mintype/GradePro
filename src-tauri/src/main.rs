use std::process::Command;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let output = Command::new("C:\\Users\\900ra\\Documents\\programming\\rust\\genesis-app\\src-tauri\\python-executables\\get_name.exe")
        .arg(name)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        format!("Hello, {}! Output from the executable: {}", name, stdout)
    } else {
        format!("Failed to greet {}: {}", name, String::from_utf8_lossy(&output.stderr))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
