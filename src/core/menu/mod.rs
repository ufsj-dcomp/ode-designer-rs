use imgui::Ui;
use crate::{App, core::plot::{CSVData, PlotLayout}};
use rfd::FileDialog;

/*
void initializePlot(int n, float n_graphs_tab){
    //plot_layout is global
    plot_layout.next_tab_id = 0;
    plot_layout.active_tabs.clear();
    plot_layout.number_of_tabs = ceil((float)n/n_graphs_tab);
    if (plot_layout.number_of_tabs == 0)
        plot_layout.number_of_tabs = 1; 
    for (int i = 0; i < plot_layout.number_of_tabs; i++){
        plot_layout.active_tabs.push_back(plot_layout.next_tab_id++);
    }
    plot_layout.style = ImPlot::GetStyle();
    plot_layout.style.LineWeight = 2.;
} */

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
                    self.simulation_state.plot_layout = PlotLayout {
                        rows: 2,
                        cols: 2,                        
                        number_of_tabs: self.simulation_state.plot.plot_data.population_count()
                                    .div_ceil(4) as u32,
                        active_tabs: vec![],
                    };
                    let layout = &mut self.simulation_state.plot_layout;
                    if layout.number_of_tabs == 0 {
                        layout.number_of_tabs = 1;
                    }
                    layout.active_tabs = (0..layout.number_of_tabs).into_iter().collect();  
                    self.simulation_state.plot.plot_graph(&self.simulation_state.plot_layout);                                                      
                    /*
                    plot_layout.style = ImPlot::GetStyle();
                    plot_layout.style.LineWeight = 2.;    */
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