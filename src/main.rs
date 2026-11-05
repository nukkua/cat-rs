use std::{env, fs, io, process::ExitCode};

fn main() -> io::Result<ExitCode> {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    let mut args = env::args().skip(1).peekable();
    let mut had_error = false;

    if args.peek().is_none() {
        io::copy(&mut stdin, &mut stdout)?;
    } else {
        for file_path in args {
            if file_path == "-" {
                io::copy(&mut stdin, &mut stdout)?;
            } else {
                let mut file = match fs::File::open(&file_path) {
                    Ok(file) => file,
                    Err(error) => {
                        eprintln!("cat-rs: {file_path}: {error}");
                        had_error = true;
                        continue;
                    }
                    
                };
                io::copy(&mut file, &mut stdout)?;
            }
        }
    }

    Ok(if had_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}
