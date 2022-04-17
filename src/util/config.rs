// crate::util::config
// Configures files on start-up

use std::env;
use std::io::Read;
use std::fs;

pub struct BinConfig {
    pub program: Vec<u8>,
}

pub struct TxtConfig {
    pub program: String,
    pub name: String,
}

pub fn binconfigure() -> BinConfig {
    let args: Vec<String> = env::args().collect();
    let name: String = args[1].clone();

    // file must be mutable here as it is being read later in the program
    // We also pass &name rather than name to prevent changing the parent scope of name
    let mut file = fs::File::open(&name).expect("Could not find filename");
    let metadata = fs::metadata(&name).expect("Unable to read metadata");

    let mut buffer = vec![0u8; metadata.len() as usize];
    // We pass &mut buffer to prevent changing the scope of buffer while still permitting modification
    file.read(&mut buffer).expect("Buffer overflow");

    BinConfig {program: buffer}
}

pub fn binconfigure_from_filename(filename: &str) -> BinConfig {
    let name: String = String::from(filename);

    // file must be mutable here as it is being read later in the program
    // We also pass &name rather than name to prevent changing the parent scope of name
    let mut file = fs::File::open(&name).expect("Could not find filename");
    let metadata = fs::metadata(&name).expect("Unable to read metadata");

    let mut buffer = vec![0u8; metadata.len() as usize];
    // We pass &mut buffer to prevent changing the scope of buffer while still permitting modification
    file.read(&mut buffer).expect("Buffer overflow");

    BinConfig {program: buffer}
}

pub fn txtconfigure() -> TxtConfig {
    let args: Vec<String> = env::args().collect();
    let name: String = args[1].clone();

    // file must be mutable here as it is being read later in the program
    // We also pass &name rather than name to prevent changing the parent scope of name
    let mut file = fs::File::open(&name).expect("Could not find filename");

    let mut buffer = String::new();
    // We pass &mut buffer to prevent changing the scope of buffer while still permitting modification
    file.read_to_string(&mut buffer).expect("Buffer overflow");

    // Remove the extension from the name (so that we can put a .bex extension on it after assembly)
    let mut name_ext_rmvd: String = name.clone();
    name_ext_rmvd.truncate(name.len() - 4);

    TxtConfig {program: buffer, name: name_ext_rmvd}
}

pub fn txtconfigure_from_filename(filename: &str) -> TxtConfig {
    let name: String = String::from(filename);

    // file must be mutable here as it is being read later in the program
    // We also pass &name rather than name to prevent changing the parent scope of name
    let mut file = fs::File::open(&name).expect("Could not find filename");

    let mut buffer = String::new();
    // We pass &mut buffer to prevent changing the scope of buffer while still permitting modification
    file.read_to_string(&mut buffer).expect("Buffer overflow");

    // Remove the extension from the name (so that we can put a .bex extension on it after assembly)
    let name_ext_rmvd: String = name[..name.len()-4].to_string();

    TxtConfig {program: buffer, name: name_ext_rmvd}
}