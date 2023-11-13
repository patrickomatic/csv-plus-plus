use csvpp::{CliArgs, Runtime, Template};
use std::path;

fn compile_template(filename: &str) {
    let cli_args = CliArgs {
        input_filename: path::Path::new(&format!("playground/{filename}")).to_path_buf(),
        output_filename: Some(path::Path::new("test.csv").to_path_buf()),
        ..Default::default()
    };
    let runtime: Runtime = Runtime::try_from(&cli_args).unwrap();
    Template::compile(&runtime).unwrap();
}

fn criterion_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("fill", |b| {
        b.iter(|| compile_template("benches_expensive_fill.csvpp"))
    });
}

criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);
