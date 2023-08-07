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
    type Error = KdlNodeError;

    #[inline]
    fn try_from(doc: &'kdl KdlDocument) -> Result<Self, Self::Error> {
        kdl_doc_try_into_outlines(doc).map(Toc)
    }
}

fn kdl_doc_try_into_outlines(doc: &KdlDocument) -> Result<Vec<UserOutline<'_>>, KdlNodeError> {
    let nodes = doc.nodes();
    let mut outlines = Vec::with_capacity(nodes.len());

    for node in nodes {
        outlines.push(node.try_into()?);
    }

    Ok(outlines)
}

impl<'kdl> TryFrom<&'kdl KdlNode> for UserOutline<'kdl> {
    type Error = KdlNodeError;

    fn try_from(node: &'kdl KdlNode) -> Result<Self, Self::Error> {
        let title = node
            .get(0)
            .and_then(|e| e.value().as_string())
            .ok_or_else(|| KdlNodeError::new(node))?;
        let page = node
            .get(1)
            .and_then(|e| e.value().as_i64())
            .ok_or_else(|| KdlNodeError::new(node))? as u32;
        let down = match node.children() {
            Some(doc) => kdl_doc_try_into_outlines(doc)?,
            None => vec![],
        };

        Ok(UserOutline { title, page, down })
    }
}

pub use error::*;
mod error {
    use kdl::KdlNode;
    use std::fmt;
    use std::iter;

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

    impl fmt::Display for KdlNodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
