use crate::gtk::Entry;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};
use gtk4::{self as gtk, Button};
use regex::Regex;

fn edit_config(value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::PathBuf::from(format!(
        "/home/{}/.var/app/org.vinegarhq.Sober/data/sober/appData/GlobalBasicSettings_13.xml",
        whoami::username()
    ));
    let content = std::fs::read_to_string(&path)?;

    let re = Regex::new(r#"<int name="FramerateCap">\d+</int>"#)?;
    let replacement = format!(r#"<int name="FramerateCap">{}</int>"#, value);
    let new_content = re.replace(&content, replacement).to_string();

    std::fs::write(&path, new_content)?;

    println!("The new FPS value is: {}", value);
    Ok(())
}
// TODO: Finish this
// fn contains_letters(str: String) -> bool {
//     str.chars().any(|c| !c.is_ascii_digit())
// }

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("dev.shavix.SoberFrame")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("SoberFrame")
            .build();

        let fps_info_text = gtk::Label::new(Some("Enter your custom FPS limit value"));

        let fps_entry = Entry::builder()
            .placeholder_text("7127")
            .width_request(200)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .input_purpose(gtk4::InputPurpose::Digits)
            .build();

        fps_entry.connect_changed(|e| {
            let txt = e.text();
            println!("Entry changed: {}", txt);
        });

        let apply_button = Button::builder()
            .label("Apply")
            .margin_top(25)
            .margin_bottom(25)
            .margin_start(25)
            .margin_end(25)
            .build();

        let value = fps_entry.clone();
        apply_button.connect_clicked(move |_| {
            let _ = edit_config(&value.text());
        });

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.set_homogeneous(true);
        container.append(&fps_info_text);
        container.append(&fps_entry);
        container.append(&apply_button);

        window.set_child(Some(&container));
        window.present();
    });

    app.run()
}
