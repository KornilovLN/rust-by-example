extern crate tar;
extern crate flate2;

use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

//--- Попытка упаковки средствами tar
//--- https://doc.rust-lang.ru/rust-cookbook/compression/tar.html
pub fn tar_pack() -> Result<(), std::io::Error> {
    println!("\n<<< md_arch: Попытка упаковки средствами tar >>>");
    println!("https://doc.rust-lang.ru/rust-cookbook/compression/tar.html\n");

    let tar_gz = File::create("arch.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("/home/leon/work/dev/rust/tut/rust-by-example", "test")?;

    Ok(())
}