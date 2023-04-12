use super::{Error, Result};
use std::borrow::Cow;
use std::path::Path;
use std::{fs, io};

/// Represents the set of URI schemes the importer supports.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UriReader<'a> {
    /// `data:[<media type>];base64,<data>`.
    Data(Option<&'a str>, &'a str),

    /// `file:[//]<absolute file path>`.
    ///
    /// Note: The file scheme does not implement authority.
    File(&'a str),

    /// `../foo`, etc.
    Relative(Cow<'a, str>),

    /// Placeholder for an unsupported URI scheme identifier.
    Unsupported,
}

impl<'a> UriReader<'a> {
    fn parse(uri: &str) -> UriReader<'_> {
        if uri.contains(':') {
            if let Some(rest) = uri.strip_prefix("data:") {
                let mut it = rest.split(";base64,");

                match (it.next(), it.next()) {
                    (match0_opt, Some(match1)) => UriReader::Data(match0_opt, match1),
                    (Some(match0), _) => UriReader::Data(None, match0),
                    _ => UriReader::Unsupported,
                }
            } else if let Some(rest) = uri.strip_prefix("file://") {
                UriReader::File(rest)
            } else if let Some(rest) = uri.strip_prefix("file:") {
                UriReader::File(rest)
            } else {
                UriReader::Unsupported
            }
        } else {
            UriReader::Relative(urlencoding::decode(uri).unwrap())
        }
    }

    pub fn read(base: Option<&Path>, uri: &str) -> Result<Vec<u8>> {
        match UriReader::parse(uri) {
            // The path may be unused in the Scheme::Data case
            // Example: "uri" : "data:application/octet-stream;base64,wsVHPgA...."
            #[allow(deprecated)]
            UriReader::Data(_, base64) => base64::decode(base64).map_err(Error::Base64),
            UriReader::File(path) if base.is_some() => read_to_end(path),
            UriReader::Relative(path) if base.is_some() => read_to_end(base.unwrap().join(&*path)),
            UriReader::Unsupported => Err(Error::UnsupportedScheme),
            _ => Err(Error::ExternalReferenceInSliceImport),
        }
    }
}

fn read_to_end<P>(path: P) -> Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    use io::Read;
    let file = fs::File::open(path.as_ref()).map_err(Error::Io)?;
    // Allocate one extra byte so the buffer doesn't need to grow before the
    // final `read` call at the end of the file.  Don't worry about `usize`
    // overflow because reading will fail regardless in that case.
    let length = file.metadata().map(|x| x.len() + 1).unwrap_or(0);
    let mut reader = io::BufReader::new(file);
    let mut data = Vec::with_capacity(length as usize);
    reader.read_to_end(&mut data).map_err(Error::Io)?;
    Ok(data)
}
