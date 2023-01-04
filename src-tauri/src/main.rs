#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::OsCommand;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn launch_server() -> OsCommand {
//     OsCommand::create_onetime_command("python server.py").execute()
// }

#[tauri::command]
fn execute_command(command: &str) -> String {
    let mut c = OsCommand::create_onetime_command(command);
    c.execute();

    return format!("{}", serde_json::to_string_pretty(&c).unwrap());
}
