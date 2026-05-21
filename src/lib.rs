// Rules
#[allow(unused_variables)]
#[allow(dead_code)]

// Libs
use std::{
    fs::{self, OpenOptions},
    io::{self, Read},
    path::Path,
    process::{self, Command},
    time::{SystemTime, UNIX_EPOCH},
};

// Print logo
pub fn logo() {
    let annihilation = r#"
    _           _             _ _       _   _             
   / \   _ __  (_) __ _ _   _| (_) __ _| |_(_) ___  _ __  
  / _ \ | '_ \ | |/ _` | | | | | |/ _` | __| |/ _ \| '_ \ 
 / ___ \| | | || | (_| | |_| | | | (_| | |_| | (_) | | | |
/_/   \_\_| |_|/ |\__, |\__,_|_|_|\__,_|\__|_|\___/|_| |_|
             |__/ |___/
    "#;
    println!("{}", annihilation);
}

// Get path
pub fn directory() -> String {
    let mut dir = String::new();
    println!("Please enter the directory path: ");
    if io::stdin().read_line(&mut dir).is_err() { eprintln!("Error: Input failed."); process::exit(1); }
    let dir = dir.trim().to_string();
    if Path::new(&dir).is_dir() { dir } else { eprintln!("Error: Directory not found."); process::exit(1) }
}

// User confirm
pub fn confirm() {
    println!("Are you sure? (y/n) : ");
    let mut answer = String::new();
    if io::stdin().read_line(&mut answer).is_err() { process::exit(1); }
    if answer.trim().eq_ignore_ascii_case("y") { println!("Starting..."); } else { process::exit(0); }
}

// Wipe files
pub fn step_1(dir: &Path) -> bool {
    let Ok(entries) = fs::read_dir(dir) else { return false; };
    let mut success = true;

    for (i, path) in entries.flatten().map(|e| e.path()).enumerate() {
        if path.is_symlink() { continue; } 
        if path.is_dir() { if !step_1(&path) { success = false; } continue; }

        // Open & check
        let Ok(mut file) = OpenOptions::new().write(true).open(&path) else { success = false; continue; };
        let Ok(meta) = file.metadata() else { success = false; continue; };
        
        // Zeroing
        if io::copy(&mut io::repeat(0).take(meta.len()), &mut file).is_err() { success = false; }
        if file.sync_all().is_err() { success = false; }

        // Truncate
        let _ = file.set_len(0); 
        let _ = file.sync_all(); 

        // Rename & rm
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let new_path = path.with_file_name(format!("{}_{}", nanos, i));
        if fs::rename(&path, &new_path).is_err() { success = false; }
        if fs::remove_file(new_path).is_err() { success = false; }
    }
    success
}

// Final clean
pub fn step_2(dir: &str, wipe_dir: bool) {
    if wipe_dir { let _ = fs::remove_dir_all(dir); }
    let mut parent = Path::new(dir).parent().unwrap_or(Path::new("/"));
    if parent.as_os_str().is_empty() { parent = Path::new("."); }
    
    // Sync dir
    if let Ok(dir_file) = fs::File::open(parent) { let _ = dir_file.sync_all(); }

    // Trim SSD
    let _ = Command::new("fstrim").arg(parent).status();
}

// Tests
#[cfg(test)]
mod tests {
    use { super::*, std::{fs, path::Path} };

    #[test]
    fn test_recursive_annihilation() {
        let dir = "test_wipe";
        let sub = "test_wipe/hidden";

        // Setup dirs
        let _ = fs::create_dir_all(sub);
        let _ = fs::write(format!("{dir}/1.txt"), b"DATA");
        let _ = fs::write(format!("{sub}/2.txt"), b"SECRET");

        // Exec wipe
        let success = step_1(Path::new(dir));
        assert!(success, "Step 1 failed");
        step_2(dir, success);

        // Check result
        assert!(!Path::new(dir).exists(), "Critical error");
    }
}