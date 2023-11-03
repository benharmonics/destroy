use std::{
  env,
  error::Error,
  fs,
  io::Write,
  path::{Path, PathBuf},
  process,
};

fn main() {
  let mut rng = rand::thread_rng();
  let (files, other): (Vec<_>, Vec<_>) = env::args()
    .skip(1)
    .map(|s| Path::new(&s).to_path_buf())
    .partition(|buf| buf.is_file());
  // Collect flags starting with '-', e.g. "-d", "--delete", "-h", "--help"
  let flags: Vec<&str> = other
    .iter()
    .filter_map(|buf| buf.to_str())
    .filter(|&s| s.starts_with('-'))
    .collect();
  if files.is_empty() && flags.is_empty() {
    usage()
  }
  if flags.iter().any(|&s| s == "-h" || s == "--help") {
    usage()
  }
  let delete = flags.iter().any(|&s| s == "-d" || s == "--delete");
  for file in files {
    for _ in 0..10 {
      shred(&file, &mut rng).unwrap_or_else(|e| eprintln!("Failed to shred file {e}"));
    }
    if delete {
      fs::remove_file(file).unwrap_or_else(|e| eprintln!("{}", e));
    }
  }
}

fn shred(buf: &PathBuf, rng: &mut impl rand::Rng) -> Result<(), Box<dyn Error>> {
  let mut data = fs::read(buf)?;
  rng.fill_bytes(&mut data);
  let mut f = fs::OpenOptions::new().write(true).open(buf)?;
  f.write_all(&data)?;
  f.flush()?;
  Ok(())
}

fn usage() {
  let buf = std::env::current_exe().expect("Failed to get executable path");
  let filename = buf
    .file_name()
    .expect("Executable should have a file name")
    .to_str()
    .expect("Executable OsStr filename should convert to str");
  println!("Usage: {filename} [OPTIONS] <FILE> [FILES...]");
  println!("-d --delete            Delete file after shredding");
  println!("-h --help              Show usage");
  process::exit(0);
}
