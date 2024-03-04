use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::config::{self, EElementType, SBackupConfig};
use crate::ui::backup::SBackupUI;
use crate::ui::recovery::{EFileAction, SRecoveryPanel};

pub fn backup(config: &SBackupConfig, details: &SBackupUI) {
    let mut backup_folder = details.folder_path.clone();
    backup_folder.push_str("/");
    backup_folder.push_str(&details.folder_name);
    fs::create_dir_all(&backup_folder).expect("Cannot create backup folder");

    config.save(backup_folder.clone());

    for element in &config.elements {
        if element.content_type == EElementType::Folder {
            copy_dir(&element.path, &backup_folder, false).expect("Error copy folder");
        } else {
            let file_name = Path::new(&element.path)
                .file_name()
                .expect("Failed to get file name from source file path");
            let destination_path = Path::new(&backup_folder).join(file_name);
            let _ = fs::copy(&element.path, destination_path);
        }
    }
}

pub fn recovery(config: &SRecoveryPanel) {
    let backup_config_path = Path::new(&config.backup_folder).join("backup_config.toml");
    let backup_config =
        config::SBackupConfig::new(backup_config_path.to_string_lossy().to_string());
    let move_elements = config.file_action == EFileAction::Moved;

    for element in backup_config.elements {
        let element_name = Path::new(&element.path).file_name().unwrap();
        let current_element_path = Path::new(&config.backup_folder).join(element_name);
        if element.content_type == EElementType::Folder {
            let old_path = Path::new(&element.path).parent().unwrap();
            copy_dir(current_element_path, old_path, move_elements).unwrap();
        } else {
            if move_elements {
                let _ = fs::rename(current_element_path, element.path);
            } else {
                let _ = fs::copy(current_element_path, element.path);
            }
        }
    }

    if move_elements {
        fs::remove_dir_all(&config.backup_folder).unwrap();
    }
}

fn copy_dir<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q, move_folder: bool) -> io::Result<()> {
    let from = from.as_ref();
    let mut to = to.as_ref().to_path_buf();
    to = to.join(from.file_name().unwrap());

    if !to.exists() {
        fs::create_dir_all(&to)?;
    }

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let to = to.join(file_name);

        if path.is_dir() {
            copy_dir(&path, &to, move_folder)?;
        } else {
            if move_folder {
                fs::rename(&path, &to)?;
            } else {
                fs::copy(&path, &to)?;
            }
        }
    }

    Ok(())
}
