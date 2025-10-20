#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![send_vibration,send_contraction,init_serial,fetch_serial_ports])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

use std::sync::Mutex;
use lazy_static::lazy_static;
use serialport::SerialPort;
use std::time::Duration;

lazy_static! {
  static ref SERIAL_PORT: Mutex<Option<Box<dyn SerialPort>>> = Mutex::new(None);
}

#[tauri::command(rename_all = "snake_case")]
fn init_serial(port_name: String, baudrate: u32) -> bool {
  match serialport::new(port_name, baudrate)
        .timeout(Duration::from_millis(1000))
        .open()
    {
        Ok(port) => {
            *SERIAL_PORT.lock().unwrap() = Some(port);
            true
        }
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn send_vibration(data: u8) -> Result<(),String>{
  let mut guard = SERIAL_PORT.lock().unwrap();
  if let Some(port) = guard.as_mut() {
      println!("Sent : {}", data);
      port.write(&[8, data]).map_err(|e| e.to_string())?;
      Ok(())
  } else {
      Err("Port non initialisé".to_string())
  }
}



#[tauri::command(rename_all = "snake_case")]
fn send_contraction(contraction: u8, part_index: u8) -> Result<(),String> {
  let mut guard = SERIAL_PORT.lock().unwrap();
  if let Some(port) = guard.as_mut() {
      println!("Sent contraction part : {}  contraction: {} ", part_index, contraction);
      port.write(&[9, contraction]).map_err(|e| e.to_string())?;
      Ok(())
  } else {
      Err("Port non initialisé".to_string())
  }
}

use serialport::available_ports;

#[derive(serde::Serialize)]
struct SerialPortInfo{
  port_name: String,
  port_type: String,
}

#[tauri::command]
fn fetch_serial_ports() -> Vec<SerialPortInfo> {
  match available_ports() {
      Ok(ports) => ports
          .into_iter()
          .map(|p| SerialPortInfo {
              port_name: p.port_name,
              port_type: format!("{:?}", p.port_type),
          })
          .collect(),
      Err(_) => vec![],
  }
}
