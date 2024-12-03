#[macro_export]
macro_rules! impl_cli_main {
    () => {
        fn main() -> ::std::process::ExitCode {
            let args = crate::args::parse_args();

            let run_result: Result<(), crate::error::Error> = run(&args);
            match run_result {
                Ok(()) => ::std::process::ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("error: {error}");
                    let mut error: &dyn ::std::error::Error = &error;
                    while let Some(source) = error.source() {
                        eprintln!("source: {source}");
                        error = source;
                    }
                    ::std::process::ExitCode::FAILURE
                }
            }
        }
    };
}
