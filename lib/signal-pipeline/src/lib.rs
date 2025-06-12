use std::path::Path;

use nalgebra::DVector;
use nearly::assert_nearly;
use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea, PathElement},
    series::LineSeries,
    style::{IntoFont, RED, WHITE},
};

pub mod io;
pub mod peak_detection;
pub mod preprocess;

pub fn nearly_eq(a: &DVector<f64>, b: &DVector<f64>) {
    a.iter()
        .zip(b.iter())
        .for_each(|(&a, &b)| assert_nearly!(a == b));
}

pub fn simple_graph_vec(path: impl AsRef<Path>, data: &DVector<f64>) {
    let path = path.as_ref();
    let root = BitMapBackend::new(path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(
            path.components()
                .last()
                .unwrap()
                .as_os_str()
                .to_string_lossy(),
            ("sans-serif", 50).into_font(),
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..data.len(), data.min()..data.max())
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(data.iter().copied().enumerate(), &RED))
        .unwrap();

    root.present().unwrap();
}
