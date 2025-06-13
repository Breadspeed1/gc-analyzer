use std::path::Path;

use nalgebra::DVector;
use nearly::assert_nearly;
use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea},
    series::{DashedLineSeries, LineSeries},
    style::{BLUE, IntoFont, RED, RGBColor, WHITE, full_palette::ORANGE},
};

use crate::peak_detection::Peak;

pub mod io;
pub mod peak_detection;
pub mod preprocess;

pub fn nearly_eq(a: &DVector<f64>, b: &DVector<f64>) {
    a.iter()
        .zip(b.iter())
        .for_each(|(&a, &b)| assert_nearly!(a == b));
}

pub fn simple_graph_vecs_with_peaks(
    path: impl AsRef<Path>,
    data: &[(&DVector<f64>, &RGBColor)],
    peaks: &[Peak],
) {
    let min = data
        .iter()
        .map(|v| v.0.min())
        .reduce(|a, b| a.min(b))
        .unwrap();

    let max = data
        .iter()
        .map(|v| v.0.max())
        .reduce(|a, b| a.max(b))
        .unwrap();

    let len = data.iter().map(|v| v.0.len()).max().unwrap();

    let path = path.as_ref();
    let root = BitMapBackend::new(path, (3840, 2160)).into_drawing_area();
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
        .build_cartesian_2d(0..len, min..max)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for (vec, color) in data {
        chart
            .draw_series(LineSeries::new(vec.iter().copied().enumerate(), color))
            .unwrap();
    }

    for peak in peaks {
        chart
            .draw_series(LineSeries::new(
                [(peak.pos as usize, 0.0), (peak.pos as usize, peak.height)].into_iter(),
                &BLUE,
            ))
            .unwrap();

        chart
            .draw_series(DashedLineSeries::new(
                [
                    ((peak.pos - peak.width / 2.) as usize, peak.height / 2.),
                    ((peak.pos + peak.width / 2.) as usize, peak.height / 2.),
                ]
                .into_iter(),
                4,
                1,
                ORANGE.into(),
            ))
            .unwrap();
    }

    root.present().unwrap();
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

pub fn simple_graph_vec_with_peaks(path: impl AsRef<Path>, data: &DVector<f64>, peaks: &[Peak]) {
    let path = path.as_ref();
    let root = BitMapBackend::new(path, (3840, 2160)).into_drawing_area();
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

    for peak in peaks {
        chart
            .draw_series(LineSeries::new(
                [(peak.pos as usize, 0.0), (peak.pos as usize, peak.height)].into_iter(),
                &BLUE,
            ))
            .unwrap();

        chart
            .draw_series(DashedLineSeries::new(
                [
                    ((peak.pos - peak.width / 2.) as usize, peak.height / 2.),
                    ((peak.pos + peak.width / 2.) as usize, peak.height / 2.),
                ]
                .into_iter(),
                4,
                1,
                ORANGE.into(),
            ))
            .unwrap();
    }

    root.present().unwrap();
}
