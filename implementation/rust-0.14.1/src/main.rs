fn main() {
  dotenv::dotenv().unwrap();

  let args = std::env::args().collect::<Vec<String>>();

  std::process::Command::new(&args[1])
    .args(&args[2..])
    .status()
    .unwrap();
}
