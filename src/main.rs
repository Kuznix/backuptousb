error[E0599]: the method `close` exists for reference `&FileChooserNative`, but its trait bounds were not satisfied
  --> src/main.rs:89:24
   |
89 |                   dialog.close();
   |                          ^^^^^
   |
  ::: /home/kuznix/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gtk4-0.10.3/src/auto/file_chooser_native.rs:14:1
   |
14 | / glib::wrapper! {
15 | |     #[doc(alias = "GtkFileChooserNative")]
16 | |     pub struct FileChooserNative(Object<ffi::GtkFileChooserNative, ffi::GtkFileChooserNativeClass>) @extends NativeDialog, @implements FileChooser;
...  |
21 | | }
   | |_- doesn't satisfy 20 bounds
   |
   = note: the following trait bounds were not satisfied:
           `FileChooserNative: gtk4::prelude::IsA<FileEnumerator>`
           which is required by `FileChooserNative: gtk4::prelude::FileEnumeratorExtManual`
           `FileChooserNative: gtk4::prelude::IsA<SocketListener>`
           which is required by `FileChooserNative: gtk4::prelude::SocketListenerExt`
           `FileChooserNative: gtk4::prelude::IsA<DtlsConnection>`
           which is required by `FileChooserNative: gtk4::prelude::DtlsConnectionExt`
           `FileChooserNative: gtk4::prelude::IsA<InputStream>`
           which is required by `FileChooserNative: gtk4::prelude::InputStreamExt`
           `FileChooserNative: gtk4::prelude::IsA<IOStream>`
           which is required by `FileChooserNative: gtk4::prelude::IOStreamExt`
           `FileChooserNative: gtk4::prelude::IsA<OutputStream>`
           which is required by `FileChooserNative: gtk4::prelude::OutputStreamExt`
           `FileChooserNative: gtk4::prelude::IsA<Socket>`
           which is required by `FileChooserNative: gtk4::prelude::SocketExt`
           `FileChooserNative: gtk4::prelude::IsA<PixbufLoader>`
           which is required by `FileChooserNative: gtk4::prelude::PixbufLoaderExt`
           `FileChooserNative: gtk4::prelude::IsA<gtk4::gdk4::Display>`
           which is required by `FileChooserNative: gtk4::prelude::DisplayExt`
           `FileChooserNative: gtk4::prelude::IsA<gtk4::Window>`
           which is required by `FileChooserNative: gtk4::prelude::GtkWindowExt`
           `&FileChooserNative: gtk4::prelude::IsA<FileEnumerator>`
           which is required by `&FileChooserNative: gtk4::prelude::FileEnumeratorExtManual`
           `&FileChooserNative: gtk4::prelude::IsA<SocketListener>`
           which is required by `&FileChooserNative: gtk4::prelude::SocketListenerExt`
           `&FileChooserNative: gtk4::prelude::IsA<DtlsConnection>`
           which is required by `&FileChooserNative: gtk4::prelude::DtlsConnectionExt`
           `&FileChooserNative: gtk4::prelude::IsA<InputStream>`
           which is required by `&FileChooserNative: gtk4::prelude::InputStreamExt`
           `&FileChooserNative: gtk4::prelude::IsA<IOStream>`
           which is required by `&FileChooserNative: gtk4::prelude::IOStreamExt`
           `&FileChooserNative: gtk4::prelude::IsA<OutputStream>`
           which is required by `&FileChooserNative: gtk4::prelude::OutputStreamExt`
           `&FileChooserNative: gtk4::prelude::IsA<Socket>`
           which is required by `&FileChooserNative: gtk4::prelude::SocketExt`
           `&FileChooserNative: gtk4::prelude::IsA<PixbufLoader>`
           which is required by `&FileChooserNative: gtk4::prelude::PixbufLoaderExt`
           `&FileChooserNative: gtk4::prelude::IsA<gtk4::gdk4::Display>`
           which is required by `&FileChooserNative: gtk4::prelude::DisplayExt`
           `&FileChooserNative: gtk4::prelude::IsA<gtk4::Window>`
           which is required by `&FileChooserNative: gtk4::prelude::GtkWindowExt`

error[E0599]: the trait `glib::object::IsA<DragSurface>` is not implemented for `FileChooserNative`
  --> src/main.rs:92:20
   |
92 |               dialog.present();
   |                      ^^^^^^^ requires `FileChooserNative` to be a GObject that can be statically cast to `DragSurface`
   |
  ::: /home/kuznix/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gtk4-0.10.3/src/auto/file_chooser_native.rs:14:1
   |
14 | / glib::wrapper! {
15 | |     #[doc(alias = "GtkFileChooserNative")]
16 | |     pub struct FileChooserNative(Object<ffi::GtkFileChooserNative, ffi::GtkFileChooserNativeClass>) @extends NativeDialog, @implements FileChooser;
...  |
21 | | }
   | |_- doesn't satisfy 10 bounds
   |
   = note: the following trait bounds were not satisfied:
           `FileChooserNative: gtk4::prelude::IsA<DragSurface>`
           which is required by `FileChooserNative: gtk4::prelude::DragSurfaceExt`
           `FileChooserNative: gtk4::prelude::IsA<Popup>`
           which is required by `FileChooserNative: gtk4::prelude::PopupExt`
           `FileChooserNative: gtk4::prelude::IsA<Toplevel>`
           which is required by `FileChooserNative: gtk4::prelude::ToplevelExt`
           `FileChooserNative: gtk4::prelude::IsA<Popover>`
           which is required by `FileChooserNative: gtk4::prelude::PopoverExt`
           `FileChooserNative: gtk4::prelude::IsA<gtk4::Window>`
           which is required by `FileChooserNative: gtk4::prelude::GtkWindowExt`
   = note: if this is your own object, use the `glib::wrapper!` macro to implement this trait: https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/macro.wrapper.html

For more information about this error, try `rustc --explain E0599`.
error: could not compile `backuptousb` (bin "backuptousb") due to 2 previous errors
[kuznix@archlinux backuptousb]$ 
