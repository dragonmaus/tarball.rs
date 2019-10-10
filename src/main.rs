use filetime::{set_file_times, FileTime};
use getopt::prelude::*;
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use std::{
    env,
    error::Error,
    fs::{self, File},
    io,
    path::Path,
    process,
};

use tarball::{Builder, Compression, Compressor, Mode};

fn main() -> ! {
    process::exit(match program() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{}", error);
            1
        }
    });
}

#[rustfmt::skip]
fn print_usage(program: &str) -> Result<i32, Box<dyn Error>> {
    println!("Usage: {} [-0123456789ghmqvx] [-I file] [-i pattern] [-o file] path [path ...]", program);
    println!("  -0       no compression");
    println!("  -1       fastest compression");
    println!("   …");
    println!("  -9       best compression");
    println!();
    println!("  -b       compress with bzip2");
    println!("  -g       compress with gzip");
    println!("  -x       compress with xz");
    println!();
    println!("  -m       create a minimal archive");
    println!();
    println!("  -o FILE  archive all arguments into FILE");
    println!();
    println!("  -i GLOB  ignore files matching gitignore pattern GLOB");
    println!("  -I FILE  ignore files matching gitignore patterns in FILE");
    println!();
    println!("  -q       suppress output");
    println!("  -v       print files being archived");
    println!();
    println!("  -h       display this help");

    Ok(0)
}

fn program() -> Result<i32, Box<dyn Error>> {
    let program = program_name("tarball");
    let mut args = program_args();
    let mut opts = Parser::new(&args, "0123456789I:bghi:mo:qvx");

    let mut compressor = Compressor::None;
    let mut filename: Option<String> = None;
    let mut ignore_files: Vec<String> = Vec::new();
    let mut ignore_globs: Vec<String> = Vec::new();
    let mut level = Compression::default();
    let mut mode = Mode::default();
    let mut verbosity = 1;

    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('I', Some(arg)) => ignore_files.push(arg),
                Opt('b', None) => compressor = Compressor::BZip,
                Opt('g', None) => compressor = Compressor::GZip,
                Opt('h', None) => return print_usage(&program),
                Opt('i', Some(arg)) => ignore_globs.push(arg),
                Opt('m', None) => mode = Mode::Minimal,
                Opt('o', Some(arg)) => filename = Some(arg),
                Opt('q', None) => verbosity -= 1,
                Opt('v', None) => verbosity += 1,
                Opt('x', None) => compressor = Compressor::XZip,
                Opt(c, None) if c.is_ascii_digit() => {
                    level = Compression::new(c.to_digit(10).unwrap())
                }
                _ => unreachable!(),
            },
        }
    }

    let mut args = args.split_off(opts.index());

    if let Some(filename) = filename {
        let file = File::create(&filename)?;
        let mut tarball = Builder::new(file, &mode, &compressor, level);

        let args = args.as_mut_slice();
        args.sort_unstable();

        let mut newest = ("", FileTime::zero());
        for arg in args {
            {
                let mtime = get_mtime(arg)?;
                if mtime >= newest.1 {
                    newest = (arg, mtime);
                }
            }

            append_tree(&mut tarball, &arg, &ignore_files, &ignore_globs, verbosity)?;
        }

        tarball.finish()?;
        update_timestamp(&filename, newest.0)?;
        if verbosity >= 1 {
            println!("{} created", &filename);
        }
    } else {
        for path in args {
            let filename = match compressor {
                Compressor::None => format!("{}.tar", path),
                Compressor::GZip => format!("{}.tar.gz", path),
                Compressor::BZip => format!("{}.tar.bz2", path),
                Compressor::XZip => format!("{}.tar.xz", path),
            };
            let file = File::create(&filename)?;
            let mut tarball = Builder::new(file, &mode, &compressor, level);

            append_tree(&mut tarball, &path, &ignore_files, &ignore_globs, verbosity)?;

            tarball.finish()?;
            update_timestamp(&filename, &path)?;
            if verbosity >= 1 {
                println!("{} created", &filename);
            }
        }
    }

    Ok(0)
}

fn append_tree<P: AsRef<Path>>(
    tarball: &mut Builder,
    path: P,
    ignore_files: &[String],
    ignore_globs: &[String],
    verbosity: i32,
) -> Result<(), Box<dyn Error>> {
    let mut walker = WalkBuilder::new(&path);
    for ignore_file in ignore_files {
        walker.add_ignore(ignore_file);
    }
    if !ignore_globs.is_empty() {
        let mut overrides = OverrideBuilder::new(&path);
        for ignore_glob in ignore_globs {
            overrides.add(&fix_glob(&ignore_glob))?;
        }
        walker.overrides(overrides.build()?);
    }
    walker.standard_filters(false);
    walker.sort_by_file_path(Path::cmp);

    for entry in walker.build() {
        let entry = entry?;
        if verbosity >= 2 {
            println!("{}", entry.path().display());
        }
        tarball.append_path(&entry.path())?;
    }

    Ok(())
}

fn fix_glob(glob: &str) -> String {
    let mut chars = glob.chars();
    if chars.next() == Some('!') {
        return chars.as_str().into();
    }
    return format!("!{}", glob);
}

fn get_mtime(path: &str) -> io::Result<FileTime> {
    let meta = fs::metadata(path)?;
    Ok(FileTime::from_last_modification_time(&meta))
}

fn program_args() -> Vec<String> {
    env::args_os()
        .map(|a| a.to_string_lossy().into_owned())
        .collect()
}

fn program_name(default: &str) -> String {
    match env::args_os().next() {
        None => default.to_string(),
        Some(os_string) => match Path::new(&os_string).file_name() {
            None => default.to_string(),
            Some(os_str) => os_str.to_string_lossy().into_owned(),
        },
    }
}

fn update_timestamp(to: &str, from: &str) -> io::Result<()> {
    let meta = fs::metadata(from)?;
    let atime = FileTime::from_last_access_time(&meta);
    let mtime = FileTime::from_last_modification_time(&meta);

    set_file_times(to, atime, mtime)
}
