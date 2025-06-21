use std::{fs::File, path::Path};

use itertools::Itertools;
use nalgebra::DVector;
use serde::Deserialize;
use serde_json::Value;

pub mod peaks;

#[derive(Deserialize, Debug)]
pub struct SignalConfig {}

pub struct SignalAnalyzer {
    data: DVector<f64>,
}

#[derive(Debug)]
pub enum ReadError<'a> {
    ParseError(serde_json::Error),
    IOError(std::io::Error),
    Other(&'a str),
}

impl SignalAnalyzer {
    pub fn new<'a>(path: impl AsRef<Path>) -> Result<Self, ReadError<'a>> {
        let file = File::open(path).map_err(ReadError::IOError)?;
        let value: Value = serde_json::from_reader(file).map_err(ReadError::ParseError)?;

        let data = value
            .get("detectors")
            .ok_or(ReadError::Other("No detectors."))?
            .as_object()
            .ok_or(ReadError::Other("Invalid data format."))?
            .iter()
            .at_most_one()
            .map_err(|_| ReadError::Other("More than one detector."))?
            .map(|(_, val)| val)
            .ok_or(ReadError::Other("No detectors."))?
            .get("values")
            .ok_or(ReadError::Other("No values read."))?
            .as_array()
            .ok_or(ReadError::Other("values property is not an array."))?
            .iter()
            .map(|v| v.as_f64())
            .collect::<Option<Vec<f64>>>()
            .ok_or(ReadError::Other("Failed to read all datapoints."))?;

        Ok(Self {
            data: DVector::from_vec(data),
        })
    }

    pub fn analyze() /* -> Result<SignalReport, SignalError> */
    {
        todo!("Do the thing.")
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_fail() {
        assert!(super::SignalAnalyzer::new("NOTAFILE").is_err());
    }
}
