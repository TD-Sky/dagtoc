use std::fmt;
use std::iter;

use kdl::{KdlDocument, KdlNode};
use mupdf::Outline;

use crate::toc;

pub trait IntoKdlDocument {
    fn into_kdl_doc(self) -> KdlDocument;
}

pub trait TryAsOutlines {
    fn try_as_outlines(&self) -> Result<Vec<toc::UserOutline<'_>>, KdlNodeError>;
}

impl<'a, I> IntoKdlDocument for I
where
    I: IntoIterator<Item = &'a Outline>,
{
    fn into_kdl_doc(self) -> KdlDocument {
        let mut doc = KdlDocument::new();
        let nodes = doc.nodes_mut();

        for outline in self {
            let mut node = KdlNode::new("-");
            node.push(outline.title.to_owned());

            if let Some(page) = outline.page {
                node.push(page as i64 + 1);
            }

            if !outline.down.is_empty() {
                node.set_children(outline.down.into_kdl_doc());
            }

            nodes.push(node);
        }

        doc
    }
}

impl TryAsOutlines for KdlDocument {
    fn try_as_outlines(&self) -> Result<Vec<toc::UserOutline<'_>>, KdlNodeError> {
        let nodes = self.nodes();
        let mut outlines = Vec::with_capacity(nodes.len());

        for node in nodes {
            outlines.push(node.try_into()?);
        }

        Ok(outlines)
    }
}

impl<'kdl> TryFrom<&'kdl KdlNode> for toc::UserOutline<'kdl> {
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
            Some(doc) => doc.try_as_outlines()?,
            None => vec![],
        };

        Ok(toc::UserOutline { title, page, down })
    }
}

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
