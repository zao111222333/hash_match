fn main() {
    // run benchmark
    for (_, bench_fn) in match_bench::BENCHS {
        bench_fn();
    }

    // collect results & visualize
    use plotly::{
        layout::{Axis, AxisType},
        ImageFormat, Layout, Plot, Scatter,
    };
    let mut plot = Plot::new();
    plot.set_layout(
        Layout::new()
            .x_axis(Axis::new().type_(AxisType::Log).title("MATCH #Arms"))
            .y_axis(Axis::new().type_(AxisType::Log).title("MATCH Time (ns)"))
            .title(format!(
                "Comparision of MATCH Performance\nPlatform: {} # {}Core\n{}",
                sysinfo::System::new_all().cpus()[0].brand(),
                sysinfo::System::new_all().cpus().len(),
                chrono::offset::Utc::now().to_rfc2822()
            )),
    );
    for method in ["match_str", "match_hash", "lookup_phf", "lookup_lazy"] {
        let times = match_bench::BENCHS
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
                .map(|f| f / *n as f64)
                .unwrap()
            })
            .collect();
        let trace =
            Scatter::new(match_bench::BENCHS.iter().map(|(n, _)| n).collect(), times).name(method);
        plot.add_trace(trace);
    }
    plot.write_image("results.svg", ImageFormat::SVG, 800, 600, 1.0);
}
