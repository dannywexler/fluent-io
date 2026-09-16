use camino::Utf8PathBuf;

#[derive(Debug)]
pub enum VideoFileActionError {
    Phash { path: Utf8PathBuf },
}

pub type VideoFileActionResult<T = ()> = Result<T, VideoFileActionError>;
