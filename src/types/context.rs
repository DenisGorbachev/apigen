use crate::{PathContents, TryFromPathBufPathContents};
use derive_more::From;
use futures::TryStreamExt;
use std::path::Path;
use subtype::subtype_string;
use thiserror::Error;

subtype_string!(
    pub struct Context(pub String);
);

impl crate::TryFromAsync<&Path> for Context {
    type Error = TryFromPathErrorForContext;

    /// The output will contain a newline at the end
    async fn try_from_async(path: &Path) -> Result<Self, TryFromPathErrorForContext> {
        let string = PathContents::stream(path)
            .map_err(TryFromPathErrorForContext::from)
            .try_fold(String::new(), fold_path_contents_res)
            .await?;
        Ok(Self(string))
    }
}

pub async fn fold_path_contents_res(mut acc: String, contents: PathContents) -> Result<String, TryFromPathErrorForContext> {
    quick_xml::se::to_writer_with_root(&mut acc, "file", &contents).unwrap();
    acc.push('\n');
    Ok(acc)
}

#[derive(Error, From, Debug)]
pub enum TryFromPathErrorForContext {
    #[error("")]
    WriteXmlFailed { source: quick_xml::Error },
    #[error("")]
    TryFromPathBufPathContentsFailed { source: TryFromPathBufPathContents },
}
