use std::process::exit;

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub fn handle_error(result: Result) {
  match result {
    Err(error) => {
      println!("ERROR: {}", error);
      exit(1)
    }
    _ => {}
  }
}
