use imgui::Ui;
use implot::ImVec4;
use std::path::Path;

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
        
        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let record = result?;
            let mut records_f64: Vec<f64> = record.iter().map(|v| v.parse::<f64>().unwrap()).collect();
            plot_data.time.push(records_f64.remove(0));
            plot_data.data.push(records_f64); 
        }
        
        Ok(plot_data)
    }

    pub fn population_count(&self) -> usize{
        self.data[0].len()
    }    
}

impl PlotInfo{
    //Chamadas para o ImPlot 
    pub fn plot_graph(&self, layout: &PlotLayout){
        /*let plotting_context = implot::Context::create();
        let plot_ui = plotting_context.get_plot_ui();
            implot::Plot::new("my title")
            .size([300.0, 200.0]) // other things such as .x_label("some_label") can be added too
            .build(&plot_ui, || {
            // Do things such as plotting lines
    }); */
    }
}