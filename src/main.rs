use std::path::Path;

use sysinfo::Disks;
use walkdir::WalkDir;
fn main() {
    // println!("Hello World");

    // // We display all disks' information:
    // println!("=> disks:");
    // if let Ok(disks) = Disks::new_with_refreshed_list() {
    //     for disk in &disks {
    //         println!("{disk:?}");
    //     }
    // }

    let mut jpg_card = Path::new("");
    let mut raw_card = Path::new("");

    let disks = Disks::new_with_refreshed_list().unwrap();
    for disk in &disks {
        if disk.is_removable() {
            println!("{:?} mounted at {:?}", disk.name(), disk.mount_point());
            // Check if the drive is full of jpgs or NEF files
            let root_path = disk.mount_point().join("DCIM");
            if root_path.exists() {
                println!("DCIM exists");
                // print files in DCIM/
                let (mut nef, mut jpg) = (0, 0);

                for entry in WalkDir::new(&root_path).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        if entry
                            .file_name()
                            .to_str()
                            .unwrap_or("")
                            .to_lowercase()
                            .ends_with("nef")
                        {
                            nef += 1;
                        }
                        if entry
                            .file_name()
                            .to_str()
                            .unwrap_or("")
                            .to_lowercase()
                            .ends_with("jpg")
                        {
                            jpg += 1;
                        }
                        println!("{}", entry.file_name().to_str().unwrap_or("ERROR"))
                    }
                }
                if jpg > nef {
                    jpg_card = disk.mount_point();
                } else {
                    raw_card = disk.mount_point();
                }
            }
        }
    }
    println!("JPG: {}", jpg_card.to_str().unwrap_or("ERROR"));
    println!("RAW: {}", raw_card.to_str().unwrap_or("ERROR"));
}
