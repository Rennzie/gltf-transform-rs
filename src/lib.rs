// #![deny(missing_docs)]
#![allow(unknown_lints)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod document;
mod math;
mod platform_io;
mod properties;

pub mod prelude {
    pub use crate::document::Document;
    pub use crate::platform_io::*;
    pub use crate::properties::prelude::*;
}
