use error_stack::{Result, ResultExt};
use ndarray::{Array1, Array2, Dim, ShapeBuilder};
use ndarray_linalg::Solve;

use crate::error::SomeError;

mod consts {
    const EPSILON: f64 = 0.1;
}

use plotters::{prelude::*, style::register_font};
use test_case::{f, real_y};

pub fn main() -> Result<(), SomeError> {
    register_font(
        "sans-serif",
        FontStyle::Normal,
        include_bytes!("../ComicMono.ttf"),
    )
    .map_err(|_| SomeError)?;

    let start = 0.0;
    let end = 3.0;
    let steps_number = 1002;
    let h = (end - start) / (steps_number as f64 - 1.0);

    let lhs = make_lhs(start, end, (steps_number, steps_number));
    let rhs = make_rhs(start, end, steps_number);

    println!("{lhs}");

    let answer = lhs
        .solve_into(rhs)
        .map_err(|e| error_stack::report!(e))
        .change_context(SomeError)?;

    // let lhs_half_h = make_lhs(start, end, (2 * steps_number, 2 * steps_number));
    // let rhs_half_h = make_rhs(start, end, 2 * steps_number);

    // let answer_half_h = lhs_half_h
    //     .solve_into(rhs_half_h)
    //     .map_err(|e| error_stack::report!(e))
    //     .change_context(SomeError)?;

    // println!("{}\n{}", answer_half_h, answer,);

    // let error = answer_half_h
    //     .iter()
    //     .skip(1)
    //     .step_by(2)
    //     .zip(&answer)
    //     .map(|(y_half_h, y_full_h)| (*y_half_h - *y_full_h).abs() / 3.0)
    //     .max_by(f64::total_cmp);

    let error = answer
        .iter()
        .enumerate()
        .map(|(i, y)| (y - real_y(start + h * i as f64)).abs())
        .max_by(f64::total_cmp);
    println!("Error: {:?}", error);

    let root = SVGBackend::new("./images/pt2_test_case.png", (800, 800)).into_drawing_area();

    root.fill(&WHITE).change_context(SomeError)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(20)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..3.0, 0.0..0.1)
        .change_context(SomeError)
        .attach_printable("Chart couldn't be built")?;

    let h = (end - start) / steps_number as f64;

    chart
        .draw_series(LineSeries::new(
            answer.iter().enumerate().map(|(i, y)| {
                (
                    start + h * i as f64,
                    (*y - real_y(start + h * i as f64)).abs(),
                )
            }),
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

    Ok(())
}

fn make_lhs(start: f64, end: f64, shape: impl ShapeBuilder<Dim = Dim<[usize; 2]>>) -> Array2<f64> {
    let mut values = Array2::default(shape);
    let h = (end - start) / (values.shape()[0] as f64 - 1.0);

    let mut i = 0;
    *values.get_mut((0, 0)).unwrap() = 1.0;
    i += 1;

    let len = values.shape()[0] - 2;

    for mut row in values.rows_mut().into_iter().skip(1).take(len) {
        *row.get_mut(i - 1).unwrap() = -k(start + (i as f64 - 0.5) * h);
        *row.get_mut(i).unwrap() = k(start + (i as f64 - 0.5) * h)
            + k(start + (i as f64 + 0.5) * h)
            + h.powi(2) * q(start + i as f64 * h);
        *row.get_mut(i + 1).unwrap() = -k(start + (i as f64 + 0.5) * h);
        i += 1;
    }

    *values.get_mut((len + 1, i)).unwrap() = 1.0 + 2.0 * k(end) / h;
    *values.get_mut((len + 1, i - 1)).unwrap() = -2.0 * k(end) / h;
    values
}

fn make_rhs(start: f64, end: f64, shape: impl ShapeBuilder<Dim = Dim<[usize; 1]>>) -> Array1<f64> {
    let mut rhs = Array1::default(shape);
    let h = (end - start) / (rhs.shape()[0] as f64 - 1.0);

    let mut i = 0;

    *rhs.get_mut(0).unwrap() = 1.2;

    i += 1;

    let len = rhs.shape()[0];
    for value in rhs.iter_mut().skip(1) {
        *value = h.powi(2) * f(start + h * i as f64);

        i += 1;
    }

    *rhs.get_mut(len - 1).unwrap() = 3.2;

    rhs
}

mod real_case {
    pub fn f(x: f64) -> f64 {
        2.0 * x.powi(3) + 1.0
    }

    // pub fn real_y(x: f64) -> f64 {
    //     panic!("Never call this function! It doesn't even make any sense to ask for a solution in a real case scenario.")
    // }
}

mod test_case {
    pub fn f(x: f64) -> f64 {
        x * f64::exp(x) / (13.0 * f64::exp(3.0) - 1.0)
            - 8.0 * f64::exp(x) / (13.0 * f64::exp(3.0) - 1.0)
            + 3.6
    }

    pub fn real_y(x: f64) -> f64 {
        2.0 * f64::exp(x) / (13.0 * f64::exp(3.0) - 1.0) + 1.2 - 2.0 / (13.0 * f64::exp(3.0) - 1.0)
    }
}

fn k(x: f64) -> f64 {
    7.5 - 0.5 * x
}

fn q(_x: f64) -> f64 {
    3.0
}
