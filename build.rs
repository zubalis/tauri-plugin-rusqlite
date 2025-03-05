const COMMANDS: &[&str] = &["open_in_memory", "open_in_path", "migration", "update", "select", "batch", "close"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();
}
