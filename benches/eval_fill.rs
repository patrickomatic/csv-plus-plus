use csvpp::{CliArgs, Compiler};
use std::path;

fn compile_template(filename: &str) {
    Compiler::try_from(&CliArgs {
        input_filename: path::Path::new(&format!("playground/benches/{filename}")).to_path_buf(),
        output_filename: Some(path::Path::new("test.csv").to_path_buf()),
        no_cache: true,
        ..Default::default()
    })
    .unwrap()
    .compile()
    .unwrap();
}

fn bench(c: &mut criterion::Criterion) {
    c.bench_function("eval_fill", |b| {
        b.iter(|| compile_template("eval_fill.csvpp"))
    });
}

criterion::criterion_group! {
    name = benches;
    config = criterion::Criterion::default()
        .with_profiler(pprof::criterion::PProfProfiler::new(100, pprof::criterion::Output::Flamegraph(None)));
    targets = bench
}

criterion::criterion_main!(benches);
