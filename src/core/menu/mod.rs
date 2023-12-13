use imgui::Ui;
use crate::App;
use rfd::FileDialog;

impl App {
    pub fn draw_menu(&mut self, ui: &Ui){
        ui.menu_bar(|| {
            ui.menu("File", || {
                if ui.menu_item("New") {
                    todo!("New not implemented");
                }

                if ui.menu_item("Load") {
                    self.load_state();
                }

                if ui.menu_item("Save") {
                    self.save_state();
                }

                if ui.menu_item("Plot CSV file") {
                    //caixa de dialogo para abrir o CSV
                    let files = FileDialog::new()
                        .add_filter("csv", &["csv"])
                        .set_directory(".")
                        .pick_file();
                    println!("{files:?}");

                    match files {
                        None => (),
                        Some(file) => { 
                            file; //atualizar o estado do App 
                        } 
                    }
                    //clear plot data 
                     
                }                
            });

            ui.menu("Edit", || {
                if ui.menu_item("Generate Code") {
                    todo!("Generate code not implemented");
                }

                if ui.menu_item("Simulate") {
                    todo!("Simulate not implemented");
                }
            })
        });
    }

}