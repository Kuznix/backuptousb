use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, CheckButton, Orientation, Box as GtkBox, Label, Window};
use gtk::{FileChooserAction, FileChooserNative};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use dirs;

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

        // Folder chooser using FileChooserNative
        let folder_chooser = FileChooserNative::builder()
            .title("Choose Target Folder")
            .action(F
