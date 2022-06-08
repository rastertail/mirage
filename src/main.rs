use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;

fn main() {
    let app = Application::builder()
        .application_id("net.rastertail.Mirage")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(1200)
            .default_height(800)
            .title("Mirage")
            .build();

        window.show();
    });

    app.run();
}
