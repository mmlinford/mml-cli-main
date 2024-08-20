#[macro_export]
macro_rules! impl_cli_main {
    () => {
        fn main_() -> ::std::process::ExitCode {
            let args = parse_args();

            match run(&args) {
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
