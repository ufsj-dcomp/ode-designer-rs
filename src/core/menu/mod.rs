use imgui::Ui;
use implot::PlotUi;
use crate::{App, core::plot::{CSVData, PlotLayout}};
use rfd::FileDialog;


impl App {    
    
    fn draw_menu_loadcsv(&mut self, ui: &Ui){
        if ui.menu_item("Plot CSV file") {
            //caixa de dialogo para abrir o CSV
            let files = FileDialog::new()
                .add_filter("csv", &["csv"])
                .set_directory(".")
                .pick_file();
            println!("{files:?}");
    
            match files {
                None => return,
                Some(file_path) => { 
                    let result = CSVData::load_data(file_path).unwrap(); 
                    println!("{:#?}", result);
                    self.simulation_state.plot.plot_data = result;
                    self.simulation_state.flag_simulation = false;
                    self.simulation_state.flag_plot_all = false;
                    self.simulation_state.plot.xlabel = String::from("time (days)");
                    self.simulation_state.plot.ylabel = String::from("conc/ml");
                    self.simulation_state.plot_layout = PlotLayout::new(2,2,
                        self.simulation_state.plot.plot_data.population_count().div_ceil(4) as u32);
                    let layout = &mut self.simulation_state.plot_layout;
                    if layout.number_of_tabs == 0 {
                        layout.number_of_tabs = 1;
                    }
                    layout.active_tabs = (0..layout.number_of_tabs).into_iter().collect();  
                    //self.simulation_state.plot.plot_graph(ui, layout);                    
                } 
            }                        
             
        }  
    }
    
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

                self.draw_menu_loadcsv(ui); 
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