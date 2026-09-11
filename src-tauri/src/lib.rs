// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// ------------------------------------------------------------
// Soporte para "Abrir con": cuando Windows lanza la app pasando
// la ruta de un archivo compatible como argumento (doble clic o
// clic derecho > Abrir con > DarkNote), la guardamos aquí al
// arrancar para que el frontend la pida una sola vez.
// ------------------------------------------------------------
struct LaunchFile(Mutex<Option<String>>);

const SUPPORTED_EXT: [&str; 15] = [
    "txt", "html", "css", "js", "json", "md", "py",
    "java", "cpp", "c", "ts", "xml", "yml", "yaml", "dnote",
];

#[tauri::command]
fn get_launch_file(state: State<LaunchFile>) -> Option<String> {
    // .take() entrega el valor una sola vez y lo deja en None,
    // para que no se recargue si luego se crea/abre otro archivo.
    state.0.lock().unwrap().take()
}

fn detect_launch_file() -> Option<String> {
    std::env::args().skip(1).find(|arg| {
        let path = std::path::Path::new(arg);
        path.is_file()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| SUPPORTED_EXT.contains(&e.to_lowercase().as_str()))
                .unwrap_or(false)
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(LaunchFile(Mutex::new(detect_launch_file())))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_launch_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
