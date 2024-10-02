use error_stack::{Result, ResultExt};
use ndarray::{Array1, Array2, Dim, ShapeBuilder};
use ndarray_linalg::Solve;
use plotters::{prelude::*, style::register_font};
use std::f64::consts::E;

use crate::error::SomeError;

mod consts {
    const EPSILON: f64 = 0.001;
}

pub fn main() -> Result<(), SomeError> {
    let steps_number = 500;
    let start = 0.0;
    let end = 3.0;
    register_font(
        "sans-serif",
        FontStyle::Normal,
        include_bytes!("../ComicMono.ttf"),
    )
    .map_err(|_| SomeError)?;

    let values = make_values(start, end, (steps_number, steps_number));

    // println!("{}", values);

    let rhs = make_rhs(start, end, steps_number);

    // println!("{}", rhs);

    let answer: Array1<_> = values
        .solve_into(rhs)
        .map_err(|err| error_stack::report!(err))
        .change_context::<SomeError>(SomeError)?;

    let real_y = make_real_y(start, end, steps_number);

    // println!("{:?}", (real_y - answer).into_iter().max_by(f64::total_cmp));

    {
        let root = SVGBackend::new("./images/pt1.svg", (800, 800)).into_drawing_area();

        root.fill(&WHITE).change_context(SomeError)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(20)
            .y_label_area_size(50)
            .build_cartesian_2d(0.0..3.0, 0.0..5.0)
            .change_context(SomeError)
            .attach_printable("Chart couldn't be built")?;

        let h = (end - start) / steps_number as f64;

        chart
            .draw_series(LineSeries::new(
                real_y
                    .iter()
                    .enumerate()
                    .map(|(i, y)| (start + h * i as f64, *y)),
                &RED,
            ))
            .change_context(SomeError)
            .attach_printable("Couldn't draw real_y")?
            .label("Real y")
            .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

        chart
            .draw_series(LineSeries::new(
                answer
                    .iter()
                    .enumerate()
                    .map(|(i, y)| (start + h * i as f64, *y)),
                &GREEN,
            ))
            .change_context(SomeError)
            .attach_printable("Couldn't draw answer")?
            .label("Answer")
            .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &GREEN));

        chart.configure_mesh().draw().change_context(SomeError)?;

        chart
            .configure_series_labels()
            .background_style(&BLACK.mix(0.1))
            .draw()
            .change_context(SomeError)?;

        root.present().change_context(SomeError)?;
    }

    let mut diff = real_y - answer;
    diff.mapv_inplace(|val| val.abs());

    {
        let root = SVGBackend::new("./images/pt1_diff.svg", (800, 800)).into_drawing_area();

        root.fill(&WHITE).change_context(SomeError)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(20)
            .y_label_area_size(50)
            .build_cartesian_2d(0.0..3.0, 0.0..0.0001)
            .change_context(SomeError)
            .attach_printable("Chart couldn't be built")?;

        let h = (end - start) / steps_number as f64;

        chart
            .draw_series(LineSeries::new(
                diff.iter().enumerate().filter_map(|(i, y)| {
                    // if *y != 0.0 {
                    Some((start + h * i as f64, *y))
                    // } else {
                    //     None
                    // }
                }),
                &BLUE,
            ))
            .change_context(SomeError)
            .attach_printable("Couldn't draw diff")?
            .label("diff")
            .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLUE));

        chart.configure_mesh().draw().change_context(SomeError)?;

        chart
            .configure_series_labels()
            .background_style(&BLACK.mix(0.1))
            .draw()
            .change_context(SomeError)?;

        root.present().change_context(SomeError)?;

        println!("{}", diff.into_iter().max_by(f64::total_cmp).unwrap());
    }

    Ok(())
}

fn f(x: f64) -> f64 {
    4.0 * x - 2.0
}

fn k(x: f64) -> f64 {
    f64::exp(x)
}

fn real_y(x: f64) -> f64 {
    2.0 * E.powi(3) / (E.powi(3) - 1.0) * E.powf(-x) - 2.0 / (E.powi(3) - 1.0) - 2.0 * x.powi(2)
        + 6.0 * x
}

fn make_values(
    start: f64,
    end: f64,
    shape: impl ShapeBuilder<Dim = Dim<[usize; 2]>>,
) -> Array2<f64> {
    let mut values = Array2::default(shape);
    let h = (end - start) / (values.shape()[0] as f64 - 1.0);

    let mut i = 0;
    *values.get_mut((0, 0)).unwrap() = 1.0;
    i += 1;

    let len = values.shape()[0] - 2;

    for mut row in values.rows_mut().into_iter().skip(1).take(len) {
        *row.get_mut(i - 1).unwrap() = -k(start + (i as f64 - 0.5) * h);
        *row.get_mut(i).unwrap() =
            k(start + (i as f64 - 0.5) * h) + k(start + (i as f64 + 0.5) * h);
        *row.get_mut(i + 1).unwrap() = -k(start + (i as f64 + 0.5) * h);
        i += 1;
    }

    *values.get_mut((len + 1, len + 1)).unwrap() = 1.0;
    values
}

fn make_rhs(start: f64, end: f64, shape: impl ShapeBuilder<Dim = Dim<[usize; 1]>>) -> Array1<f64> {
    let mut rhs = Array1::default(shape);
    let h = (end - start) / (rhs.shape()[0] as f64 - 1.0);

    let mut i = 0;

    *rhs.get_mut(0).unwrap() = 2.0;

    i += 1;

    let len = rhs.shape()[0];
    for value in rhs.iter_mut().skip(1) {
        *value = h.powi(2) * f(start + h * i as f64) * f64::exp(start + h * i as f64);

        i += 1;
    }

    *rhs.get_mut(len - 1).unwrap() = 0.0;

    rhs
}

fn make_real_y(
    start: f64,
    end: f64,
    shape: impl ShapeBuilder<Dim = Dim<[usize; 1]>>,
) -> Array1<f64> {
    let mut real_y_values = Array1::default(shape);
    let h = (end - start) / (real_y_values.shape()[0] as f64 - 1.0);

    for (i, value) in real_y_values.iter_mut().enumerate() {
        *value = real_y(start + h * i as f64);
    }

    real_y_values
}
