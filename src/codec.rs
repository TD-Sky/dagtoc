use std::num::NonZeroU32;

use anyhow::bail;
use kdl::{KdlDocument, KdlNode};
use mupdf::Outline;

use crate::toc::Toc;
use crate::toc::UserOutline;

impl From<&Toc<Outline>> for KdlDocument {
    #[inline]
    fn from(toc: &Toc<Outline>) -> Self {
        outlines_into_kdl_doc(&toc.0)
    }
}

fn outlines_into_kdl_doc(outlines: &[Outline]) -> KdlDocument {
    let mut doc = KdlDocument::new();
    let nodes = doc.nodes_mut();

    for outline in outlines {
        let mut node = KdlNode::new("-");
        node.push(outline.title.to_owned());

        if let Some(page) = outline.page {
            node.push(page as i64 + 1);
        }

        if !outline.down.is_empty() {
            node.set_children(outlines_into_kdl_doc(&outline.down));
        }

        nodes.push(node);
    }

    doc
}

impl<'kdl> TryFrom<&'kdl KdlDocument> for Toc<UserOutline<'kdl>> {
    type Error = anyhow::Error;

    #[inline]
    fn try_from(doc: &'kdl KdlDocument) -> Result<Self, Self::Error> {
        kdl_doc_try_into_outlines(doc).map(Toc)
    }
}

fn kdl_doc_try_into_outlines(doc: &KdlDocument) -> anyhow::Result<Vec<UserOutline<'_>>> {
    let nodes = doc.nodes();
    let mut outlines = Vec::with_capacity(nodes.len());

    for node in nodes {
        outlines.push(node.try_into()?);
    }

    Ok(outlines)
}

impl<'kdl> TryFrom<&'kdl KdlNode> for UserOutline<'kdl> {
    type Error = anyhow::Error;

    fn try_from(node: &'kdl KdlNode) -> Result<Self, Self::Error> {
        let title = node
            .get(0)
            .and_then(|e| e.value().as_string())
            .ok_or_else(|| KdlNodeError::new(node))?;
        let page = node
            .get(1)
            .and_then(|e| e.value().as_i64())
            .ok_or_else(|| KdlNodeError::new(node))?;
        let page = if page > 0 {
            unsafe { NonZeroU32::new_unchecked(page as u32) }
        } else {
            bail!("`{page}` is not a valid page number");
        };
        let down = node
            .children()
            .map(kdl_doc_try_into_outlines)
            .transpose()?
            .unwrap_or_default();

        Ok(UserOutline { title, page, down })
    }
}

pub use error::*;
mod error {
    use std::iter;

    use kdl::KdlNode;

    #[derive(Debug)]
    pub struct KdlNodeError(String);

    impl KdlNodeError {
        pub fn new(node: &KdlNode) -> Self {
            Self(
                iter::once(node.name().value().to_owned())
                    .chain(node.entries().iter().map(|e| e.to_string()))
                    .collect::<String>(),
            )
        }
    }

    impl std::fmt::Display for KdlNodeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "KDL node error: `{}`", self.0.trim())
        }
    }

    impl std::error::Error for KdlNodeError {
        #[inline]
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }
    }
}
