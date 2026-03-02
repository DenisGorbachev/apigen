use futures::Stream;
use futures::stream::empty;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Serialize, Deserialize, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct PathContents {
    pub path: PathBuf,
    pub contents: String,
}

impl PathContents {
    pub fn stream(_dir: &Path) -> impl Stream<Item = Result<Self, TryFromPathBufPathContents>> {
        // TODO
        empty()
    }
}

impl TryFrom<PathBuf> for PathContents {
    type Error = TryFromPathBufPathContents;

    fn try_from(_path: PathBuf) -> Result<Self, Self::Error> {
        // TODO: Read contents from path
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum TryFromPathBufPathContents {
    #[error("failed to read the file at '{}'", path.display())]
    ReadFailed { path: PathBuf },
}

impl TryFrom<&Path> for PathContents {
    type Error = <Self as TryFrom<PathBuf>>::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
