#[tauri::command]
pub fn test_command(message: String) -> Result<String, String> {
    println!("Received message: {}", message);
    Ok(format!("Message received: {}", message))
}
