use std::{fs::OpenOptions, io::Write, os::unix::fs::OpenOptionsExt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1.
    // Open (or create if non-existent) file.
    // Overwrite if it exists.
    let mut fd = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o700)
        .open("/tmp/file")?;
  
    // 2.
    // Attempt to write to the file;
    let bytes_to_write = b"hello world\n";
    let bytes_written = fd.write(&bytes_to_write[..12])?;

    println!("Bytes written {bytes_written}");

    // 3.
    // No need to close the file since File implements Drop trait which cleans up when File
    // goes out of scope.
    Ok(())
}

