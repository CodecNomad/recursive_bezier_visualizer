use charming::{component::Axis, element::AxisType, series::Line, Chart, HtmlRenderer};
use clap::Parser;
use num_integer::binomial;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_delimiter = ',')]
    points: Vec<f64>,

    #[arg(short, long, default_value_t = 100)]
    samples: usize,

    #[arg(short, long, default_value = "bezier_chart.html")]
    output: PathBuf,

    #[arg(long, default_value_t = 1000)]
    width: u32,

    #[arg(long, default_value_t = 800)]
    height: u32,
}

pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub fn calculate_recursive_bezier(control_points: &[f64], time: f64) -> f64 {
    let size = control_points.len();
    let delta_time = 1f64 - time;
    control_points
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let binom = binomial(size - 1, i);
            let term = binom as f64 * delta_time.powi((size - 1 - i) as i32) * time.powi(i as i32);
            term * val
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.points.len() < 2 {
        return Err("At least two control points are required".into());
    }

    let bezier_data: Vec<Vec<f64>> = (0..cli.samples)
        .map(|i| {
            let time = i as f64 / (cli.samples - 1) as f64;
            let y = calculate_recursive_bezier(&cli.points, time);
            vec![time, y]
        })
        .collect();

    let chart = Chart::new()
        .x_axis(Axis::new().type_(AxisType::Value).name("Time"))
        .y_axis(Axis::new().type_(AxisType::Value).name("Bezier Value"))
        .series(Line::new().name("Bezier Curve").data(bezier_data));

    let mut renderer = HtmlRenderer::new("Bezier Curve", cli.width.into(), cli.height.into());
    renderer.render(&chart)?;
    renderer.save(&chart, &cli.output)?;

    println!("Bezier curve chart saved to {}", cli.output.display());

    Ok(())
}
