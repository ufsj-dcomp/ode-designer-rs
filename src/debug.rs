use imgui::Ui;
pub fn draw(ui: &Ui) {
    if ui.button("Error") {
        log::error!("This is the message body");
        log::warn!("This is the message body");
        log::info!("This is the message body");
        log::debug!("This is the message body");
        log::trace!("This is the message body");
    }
}

