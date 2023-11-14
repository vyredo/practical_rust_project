extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Image, Label,
    Orientation};
use std::env;

fn main(){
    let app = Application::new(
        Some("com.vidy.catsay-gtk"),
        gio::ApplicationFlags::empty()
    ).expect("Failed to initialize GTK.");

    app.connect_startup(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay GTK");
        window.set_default_size(500,500);
        
        let layout_box = Box::new(Orientation::Vertical, 0);
        let label = Label::new(Some("Hello, world!"));
        layout_box.add(&label);

        let cat_image = Image::new_from_file("./cat_image.jpg");
        layout_box.add(&cat_image);
        
        window.add(&layout_box);
        window.show_all();



        window.connect_delete_event(|win, _| {
            win.destroy();
            Inhibit(false)
        });
        
    });

    app.connect_activate(|_| {});   
    app.run(&env::args().collect::<Vec<_>>());
}