use imgui::{Ui, TabBar, TabItem};
use implot::{ImVec4, PlotLine, PlotUi};
use nalgebra::DMatrix;
use std::path::Path;

use super::App;

#[derive(Default,Debug)]
pub struct CSVData {
    labels: Vec<String>,
    data: Vec<Vec<f64>>,
    time: Vec<f64>
}

#[derive(Default)]
pub struct PlotInfo { 
    pub title: String,
    pub xlabel: String, 
    pub ylabel: String,
    pub plot_data: CSVData    
}

#[derive(Default)]
pub struct PlotLayout {
    pub rows: u32,
    pub cols: u32,
    pub number_of_tabs: u32,
    pub active_tabs: Vec<u32>,
}

const COLORS: &[ImVec4] = &[
    ImVec4::new(0.98,0.027,0.027,1.0),
    ImVec4::new(0.09,0.027,0.98,1.0),
    ImVec4::new(0.98,0.027,0.027,1.0), //vermelha
    ImVec4::new(0.09,0.027,0.98,1.0), 
    ImVec4::new(0.027,0.98,0.12,1.0), //verde claro
    ImVec4::new(0.96,0.98,0.027,1.0), //amarelo
    ImVec4::new(0.5,0.,1.0,1.0), //roxo
    ImVec4::new(1.0,0.5,0.,1.0), //laranja
    ImVec4::new(0.2,1.0,1.0,1.0), //ciano
    ImVec4::new(1.0,0.2,0.6,1.0), //rosa
    ImVec4::new(0.4,0.7,1.0,1.0), //azul claro
    ImVec4::new(1.0,0.4,0.4,1.0), //vermelho claro                                                        
    ImVec4::new(1.0,0.2,1.0,1.0), //magenta                            
    ImVec4::new(1.0,0.7,0.4,1.0), //laranja claro
    ImVec4::new(0.74,0.055,0.055,1.0),
    ImVec4::new(0.6,0.298,0.,1.0),
    ImVec4::new(0.1,0.4,0.1,1.0)  //verde escuro 
];

impl CSVData {
    pub fn load_data<A: AsRef<Path>>(file_path: A) -> std::io::Result<Self>{    
        let fp = std::fs::File::open(file_path)?;
        let mut rdr = csv::Reader::from_reader(fp);

        let mut plot_data = CSVData::default();
        
        if rdr.has_headers() {
            plot_data.labels = rdr.headers().unwrap().iter().map(|v| v.to_string()).collect();
        }
        println!("{:?}", plot_data.labels);
        let n_cols = plot_data.labels.len();

        let mut populations : Vec<Vec<f64>> = (0..n_cols).map(|_| Vec::new()).collect();
        
        for result in rdr.records() {
            
            let record = result?;
            record
                .iter()
                .map(|v| v.parse::<f64>().unwrap())
                .zip(populations.iter_mut())
                .for_each(|(value,population)| {
                    population.push(value);
                });
        }
        plot_data.time = populations.remove(0);
        plot_data.data = populations;
        
        Ok(plot_data)
    }

    pub fn population_count(&self) -> usize{
        self.data[0].len()
    }    
}

impl PlotLayout {
    pub fn new(r: u32, c: u32, n_tabs: u32) -> Self {
        Self {
            rows: r,
            cols: c,
            number_of_tabs: n_tabs,
            active_tabs: vec![],
        }
    }
}

impl PlotInfo{
    //Chamadas para o ImPlot 
    pub fn plot_graph(&self, ui: &Ui, layout: &PlotLayout){
        
        /*let tab_bar: imgui::TabBar<&str> = imgui::TabBar::new("Tabs");
        tab_bar.build(&ui, || {
            imgui::TabItem::new("Tab 0").build(&ui, || {
                let x = vec![1.0,2.0,3.0,4.0,5.0];
                let y = vec![1.0,2.0,3.0,4.0,5.0];
                implot::Plot::new("Plot")
                    .size([300.0, 200.0]) // other things such as .x_label("some_label") can be added too
                    .build(&plot_ui, || {
                    PlotLine::new("legend label").plot(&self.plot_data.time,&data);
                });
                
            });
        });*/

        /*//let content_width = ui.window_content_region_min()/ui.window_content_region_max();              
            int colors_size = colors.len();
            for (int i = 0; i < layout.rows*plot_layout.cols; ++i) {
                int index = id + i;
                PlotLine::new(plot.labels[index])).plot(&self.time_data,);           
                }
            }
        */
        
    }
}