use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, CheckButton, Orientation, Box as GtkBox, Label, Window};
use gtk::{FileChooserAction, FileChooserDialog};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use dirs;
use std::rc::Rc;
use std::cell::RefCell;

// Include compile-time year from build.rs
include!(concat!(env!("OUT_DIR"), "/compile_year.rs"));

// Type alias for error handling with fs_extra
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
            .default_height(300)
            .build();

        // Vertical container
        let vbox = GtkBox::new(Orientation::Vertical, 10);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);
        vbox.set_margin_top(10);
        vbox.set_margin_bottom(10);

        // Instead of packing a FileChooserNative (which is a dialog, not a widget),
        // provide a button that opens a FileChooserDialog and a label showing the chosen path.
        let choose_button = Button::with_label("Choose Target Folder");
        let path_label = Label::new(Some("No folder selected"));

        // Shared state for the selected target path
        let target_path: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));

        // Checkboxes
        let ssh_checkbox = CheckButton::with_label("Copy SSH keys (~/.ssh)");
        let delete_checkbox = CheckButton::with_label("Delete files >1GB");
        let log_checkbox = CheckButton::with_label("Add log to target");

        // Buttons
        let backup_button = Button::with_label("Start Backup");
        let about_button = Button::with_label("About");

        // Pack widgets
        vbox.append(&choose_button);
        vbox.append(&path_label);
        vbox.append(&ssh_checkbox);
        vbox.append(&delete_checkbox);
        vbox.append(&log_checkbox);
        vbox.append(&backup_button);
        vbox.append(&about_button);

        window.set_child(Some(&vbox));

        // Choose button opens a FileChooserDialog
        let target_path_clone = target_path.clone();
        let path_label_clone = path_label.clone();
        let window_clone_for_chooser = window.clone();
        choose_button.connect_clicked(move |_| {
            // Use FileChooserDialog (a Dialog/Toplevel) so present() and close() are available.
            let dialog = FileChooserDialog::new(
                Some("Choose Target Folder"),
                Some(&window_clone_for_chooser),
                FileChooserAction::SelectFolder,
                &[("Cancel", gtk::ResponseType::Cancel), ("Open", gtk::ResponseType::Accept)],
            );

            dialog.set_modal(true);

            let target_path_inner = target_path_clone.clone();
            let path_label_inner = path_label_clone.clone();
            dialog.connect_response(move |dialog, response| {
                if response == gtk::ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            *target_path_inner.borrow_mut() = Some(path.clone());
                            path_label_inner.set_text(&path.to_string_lossy());
                        }
                    }
                }
                dialog.close();
            });

            dialog.present();
        });

        // Backup button clicked
        let target_path_clone2 = target_path.clone();
        let ssh_checkbox_clone = ssh_checkbox.clone();
        let delete_checkbox_clone = delete_checkbox.clone();
        let log_checkbox_clone = log_checkbox.clone();

        backup_button.connect_clicked(move |_| {
            if let Some(target_path) = target_path_clone2.borrow().clone() {
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
        let window_clone = window.clone();
        about_button.connect_clicked(move |_| {
            show_about(&window_clone);
        });

        window.show();
    });

    app.run();
}

/// Copy home folder with options
fn copy_home(target: &Path, copy_ssh: bool, delete_large: bool, log: bool) -> Result<()> {
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;

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
            fs_extra::dir::copy(&ssh_src, &ssh_dst, &fs_extra::dir::CopyOptions::new())
                .map_err(|e| Box::<dyn std::error::Error>::from(e))?;
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
        if file_name == ".ssh" {
            continue;
        }

        let target_path = target.join(&file_name);

        if path.is_dir() {
            fs::create_dir_all(&target_path)?;
            fs_extra::dir::copy(&path, &target_path, &fs_extra::dir::CopyOptions::new())
                .map_err(|e| Box::<dyn std::error::Error>::from(e))?;
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
fn show_about(parent: &impl IsA<Window>) {
    use gtk::{Dialog, Image, Box as GtkBox, Orientation};

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
    let dialog = Dialog::builder()
        .title("About BackupToUSB")
        .transient_for(parent)
        .modal(true)
        .build();

    dialog.add_buttons(&[("OK", gtk::ResponseType::Ok)]);

    // Vertical box for image + text
    let vbox = GtkBox::new(Orientation::Vertical, 10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);

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

    // Close dialog on response
    dialog.connect_response(|dialog, _| {
        dialog.close();
    });

    // Show the dialog
    dialog.present();
}
