use plotters::style::full_palette::*;
use std::iter::zip;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
    // run benchmark
    for (_, bench_fn) in BENCHS {
        bench_fn();
    }
    // collect results & visualize
    show()
}
fn show() {
    // collect results
    let mut time_min = f32::MAX;
    let mut time_max = f32::MIN;
    let res = [
        ("match_str", LIGHTGREEN_600),
        ("match_hash", RED_500),
        ("lookup_phf", YELLOW_800),
        ("lookup_lazy", LIGHTBLUE_800),
    ]
    .into_iter()
    .map(|(method, color)| {
        let times = BENCHS
            .iter()
            .map(|(n, _)| {
                std::fs::read_to_string(
                    std::path::Path::new("target/criterion")
                        .join(format!("{method}_{n}"))
                        .join("new/estimates.json"),
                )
                .ok()
                .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
                .map(|v| v["mean"]["point_estimate"].clone())
                .and_then(|v| {
                    if let serde_json::Value::Number(n) = v {
                        n.as_f64()
                    } else {
                        None
                    }
                })
                .map(|f| {
                    let time = f as f32 / *n as f32;
                    time_min = time_min.min(time);
                    time_max = time_max.max(time);
                    time
                })
                .unwrap()
            })
            .collect::<Vec<_>>();
        (method, color, times)
    })
    .collect::<Vec<_>>();
    // visualize
    use plotters::prelude::*;
    let root = SVGBackend::new("bench.svg", (800, 500)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    root.draw(&Text::new(
        format!(
            "{} {} Cores",
            sysinfo::System::new_all().cpus()[0].brand(),
            sysinfo::System::new_all().cpus().len()
        ),
        (10, 10),
        ("Arial", 15).into_font().color(&GREY_600),
    ))
    .unwrap();
    root.draw(&Text::new(
        format!("{}", chrono::offset::Utc::now().format("%d/%m/%Y %H:%M")),
        (10, 25),
        ("Arial", 15).into_font().color(&GREY_600),
    ))
    .unwrap();

    let root = root.margin(10, 10, 10, 10);
    time_min *= 0.8;
    time_max *= 1.3;
    let x_min = BENCHS.first().unwrap().0 as f32;
    let x_max = BENCHS.last().unwrap().0 as f32;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Comparison of MATCH Methods",
            ("Arial", 35).into_font().style(FontStyle::Bold),
        )
        .x_label_area_size(50)
        .y_label_area_size(65)
        .margin_right(20)
        .build_cartesian_2d((x_min..x_max).log_scale(), (time_min..time_max).log_scale())
        .unwrap();
    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .x_label_formatter(&|x| format!("{:.0}", x))
        .y_label_formatter(&|x| format!("{:.0}", x))
        .x_label_style(("Arial", 20).into_font().style(FontStyle::Bold))
        .y_label_style(("Arial", 20).into_font().style(FontStyle::Bold))
        .x_desc("MATCH #Arm")
        .y_desc("Time/OP (ns)")
        .axis_desc_style(("Arial", 25).into_font().style(FontStyle::Bold))
        .draw()
        .unwrap();
    chart
        .draw_series(std::iter::once(Rectangle::new(
            [(x_min, time_max), (x_max, time_min)],
            GREY_700.mix(0.1).filled(),
        )))
        .unwrap();
    chart
        .draw_series(std::iter::once(Rectangle::new(
            [(x_min, time_max), (x_max, time_min)],
            BLACK.stroke_width(2),
        )))
        .unwrap();
    for (method, color, times) in res {
        chart
            .draw_series(LineSeries::new(
                zip(BENCHS, times.clone())
                    .into_iter()
                    .map(|((n, _), time)| (*n as f32, time)),
                color.stroke_width(3),
            ))
            .unwrap()
            .label(method)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(3))
            });
        chart
            .draw_series(PointSeries::of_element(
                zip(BENCHS, times)
                    .into_iter()
                    .map(|((n, _), time)| (*n as f32, time)),
                5,
                color.stroke_width(3),
                &|c, s, st| {
                    return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                    + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established);
                },
            ))
            .unwrap();
    }
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("Arial", 20).into_font().style(FontStyle::Bold))
        .position(SeriesLabelPosition::LowerRight)
        .draw()
        .unwrap();
    root.present().unwrap();
}
