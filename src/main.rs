use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, CheckButton, FileChooserButton, Orientation, Box as GtkBox, Label};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use dirs;

// Include compile-time year from build.rs
include!(concat!(env!("OUT_DIR"), "/compile_year.rs"));

fn main() {
    let app = Application::builder()
        .application_id("com.Kuznix.BackupToUSB")
        .build();

    app.connect_activate(|app| {
        // Main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("BackupToUSB")
            .default_width(500)
            .default_height(250)
            .build();

        // Vertical container
        let vbox = GtkBox::new(Orientation::Vertical, 10);
        vbox.set_margin_all(10);

        // Target folder chooser
        let folder_label = Label::new(Some("Select target folder:"));
        let folder_chooser = FileChooserButton::builder()
            .title("Choose Target Folder")
            .action(gtk::FileChooserAction::SelectFolder)
            .build();

        // Checkboxes
        let ssh_checkbox = CheckButton::with_label("Copy SSH keys (~/.ssh)");
        let delete_checkbox = CheckButton::with_label("Delete files >1GB");
        let log_checkbox = CheckButton::with_label("Add log to target");

        // Buttons
        let backup_button = Button::with_label("Start Backup");
        let about_button = Button::with_label("About");

        // Pack widgets
        vbox.append(&folder_label);
        vbox.append(&folder_chooser);
        vbox.append(&ssh_checkbox);
        vbox.append(&delete_checkbox);
        vbox.append(&log_checkbox);
        vbox.append(&backup_button);
        vbox.append(&about_button);

        window.set_child(Some(&vbox));

        // Backup button clicked
        let folder_chooser_clone = folder_chooser.clone();
        let ssh_checkbox_clone = ssh_checkbox.clone();
        let delete_checkbox_clone = delete_checkbox.clone();
        let log_checkbox_clone = log_checkbox.clone();

        backup_button.connect_clicked(move |_| {
            if let Some(target_path) = folder_chooser_clone.file().map(PathBuf::from) {
                let copy_ssh = ssh_checkbox_clone.is_active();
                let delete_large = delete_checkbox_clone.is_active();
                let log = log_checkbox_clone.is_active();
                match copy_home(&target_path, copy_ssh, delete_large, log) {
                    Ok(_) => println!("Backup completed!"),
                    Err(e) => eprintln!("Backup failed: {}", e),
                }
            } else {
                eprintln!("No target folder selected!");
            }
        });

        // About button clicked
        about_button.connect_clicked(move |_| {
            show_about(&window);
        });

        window.show();
    });

    app.run();
}

/// Copy home folder with options
fn copy_home(target: &Path, copy_ssh: bool, delete_large: bool, log: bool) -> std::io::Result<()> {
    let home = dirs::home_dir().expect("Cannot find home directory");

    // Ensure target exists
    if !target.exists() {
        fs::create_dir_all(target)?;
    }

    // Copy .ssh folder if requested
    if copy_ssh {
        let ssh_src = home.join(".ssh");
        let ssh_dst = target.join(".ssh");
        if ssh_src.exists() {
            fs::create_dir_all(&ssh_dst)?;
            fs_extra::dir::copy(&ssh_src, &ssh_dst, &fs_extra::dir::CopyOptions::new())?;
        }
    }

    // Prepare log file if logging enabled
    let mut log_file = if log {
        Some(fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(target.join("backup.log"))?)
    } else {
        None
    };

    // Iterate home directory
    for entry in fs::read_dir(&home)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();

        // Skip .ssh if already copied
        if path.file_name().unwrap() == ".ssh" {
            continue;
        }

        let target_path = target.join(&file_name);

        if path.is_dir() {
            fs::create_dir_all(&target_path)?;
            fs_extra::dir::copy(&path, &target_path, &fs_extra::dir::CopyOptions::new())?;
        } else {
            // Delete large files if option enabled
            if delete_large && fs::metadata(&path)?.len() > 1_000_000_000 {
                fs::remove_file(&path)?;
                if let Some(f) = &mut log_file {
                    writeln!(f, "Deleted large file: {:?}", path)?;
                }
                continue;
            }
            fs::copy(&path, &target_path)?;
        }

        if let Some(f) = &mut log_file {
            writeln!(f, "Copied: {:?}", path)?;
        }
    }

    Ok(())
}

/// Show about dialog with logo, version, and dynamic year
fn show_about(parent: &gtk::Window) {
    use gtk::{Dialog, Image, Label, Box as GtkBox, Orientation};

    // Get version from Cargo.toml
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    // Determine year display starting from 2026
    let year_text = if COMPILE_YEAR == 2026 {
        "2026".to_string()
    } else if COMPILE_YEAR == 2027 {
        "2026-2027".to_string()
    } else {
        format!("2026-{}", COMPILE_YEAR)
    };

    // Create a dialog
    let dialog = Dialog::with_buttons(
        Some("About BackupToUSB"),
        Some(parent),
        gtk::DialogFlags::MODAL,
        &[("OK", gtk::ResponseType::Ok)],
    );

    // Vertical box for image + text
    let vbox = GtkBox::new(Orientation::Vertical, 10);
    vbox.set_margin_all(10);

    // Load logo image
    let logo_path = "extra/logo/backuptousb.png";
    let image = Image::from_file(logo_path);

    // Text label with version + year
    let text = Label::new(Some(&format!("BackupToUSB {} - {}", VERSION, year_text)));
    text.set_justify(gtk::Justification::Center);

    // Pack image and text into vbox
    vbox.append(&image);
    vbox.append(&text);

    // Add vbox to dialog content
    dialog.content_area().append(&vbox);

    // Run dialog
    dialog.run();
    dialog.close();
}
