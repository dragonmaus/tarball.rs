use std::{fs::File, io, path::Path};

use crate::{Compression, Compressor, Mode, Output};

pub struct Builder {
    inner: tar::Builder<Output>,
}

impl Builder {
    pub fn new(file: File, mode: &Mode, follow_symlinks: bool, compressor: &Compressor, level: Compression) -> Builder {
        let mut builder = tar::Builder::new(Output::new(file, compressor, level));
        builder.follow_symlinks(follow_symlinks);
        builder.mode(match mode {
            Mode::Minimal => tar::HeaderMode::Deterministic,
            Mode::Normal => tar::HeaderMode::Complete,
        });

        Builder { inner: builder }
    }

    pub fn finish(self) -> io::Result<()> {
        self.inner.into_inner()?.finish()
    }

    pub fn append_path<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        self.inner.append_path(path)
    }
}
