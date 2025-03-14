mod subnet_calculator;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![subnet_calculator::calculate_subnet_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
