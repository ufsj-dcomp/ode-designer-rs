use std::io::Read;

#[derive(Default, Debug)]
pub struct CSVData {
    pub labels: Vec<String>,
    pub lines: Vec<Vec<f64>>,
    pub time: Vec<f64>,
}

impl CSVData {

    pub fn load_data(reader: impl Read) -> std::io::Result<Self> {
        let mut rdr = csv::Reader::from_reader(reader);

        let mut data = CSVData::default();

        if rdr.has_headers() {
            data.labels = rdr
                .headers()
                .unwrap()
                .iter()
                .map(|v| v.to_string())
                .collect();
        }

        let n_cols = data.labels.len();

        let mut populations: Vec<Vec<f64>> = (0..n_cols).map(|_| Vec::new()).collect();

        for record in rdr.records() {
            //println!("record = {:?}", record);
            record?
                .into_iter()
                .map(|v| v.trim())
                .map(|v| v.parse())
                .zip(populations.iter_mut())
                .try_for_each(|(value, population)| {
                    //println!("value: {:?}", value);
                    population.push(value?);
                    Result::<(), std::num::ParseFloatError>::Ok(())
                })
                .unwrap();
        }

        data.time = populations.remove(0);
        data.lines = populations;
        data.labels.remove(0);

        Ok(data)
    }

    pub fn population_count(&self) -> usize {
        self.lines[0].len()
    }
}