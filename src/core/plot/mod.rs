use imgui::Ui;

#[derive(Default)]
pub struct PlotInfo {
    num_of_lines: i32,
    num_of_cols: i32,
    labels: Vec<String>,
    title: String,
    xlabel: String, 
    ylabel: String,
    data: Vec<Vec<f64>>,
    time: Vec<f64>,
}

#[derive(Default)]
pub struct PlotLayout {
    rows: i32,
    cols: i32,
    next_tab_id: i32,
    number_of_tabs: i32,
    active_tabs: Vec<i32>,
    //style: , 
}