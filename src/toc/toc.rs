use std::num::NonZero;

use pdf_oxide::{Destination, OutlineItem};

use super::{PageOutbound, TocError};

#[derive(Debug)]
pub struct Toc<O>(pub(crate) Vec<O>);

#[derive(Debug)]
pub struct UserOutline<'toc> {
    pub title: &'toc str,
    pub page: NonZero<usize>,
    pub children: Vec<UserOutline<'toc>>,
}

impl From<Toc<UserOutline<'_>>> for Vec<OutlineItem> {
    fn from(user: Toc<UserOutline>) -> Self {
        user.0.iter().map(OutlineItem::from).collect()
    }
}

impl From<&UserOutline<'_>> for OutlineItem {
    fn from(user: &UserOutline) -> Self {
        Self {
            title: user.title.into(),
            dest: Some(Destination::PageIndex(user.page.get() - 1)),
            children: user.children.iter().map(|item| item.into()).collect(),
        }
    }
}

impl Toc<UserOutline<'_>> {
    pub fn verify(&self) -> Result<(), TocError> {
        fn verify_rec(outlines: &[UserOutline<'_>], pre_page: &mut usize) -> Result<(), TocError> {
            for outline in outlines {
                if outline.page.get() < *pre_page {
                    return Err(TocError::new(outline.title, *pre_page));
                }

                *pre_page = outline.page.get();

                if !outline.children.is_empty() {
                    verify_rec(&outline.children, pre_page)?;
                }
            }

            Ok(())
        }

        verify_rec(&self.0, &mut 0)
    }

    pub fn offset_pages(&mut self, count: isize) -> Result<(), PageOutbound> {
        fn offset_pages_rec(
            outlines: &mut [UserOutline<'_>],
            count: isize,
        ) -> Result<(), PageOutbound> {
            for outline in outlines {
                let page = outline.page.get() as isize;

                let new_page = page + count;
                if new_page > 0 {
                    outline.page = unsafe { NonZero::new_unchecked(new_page as usize) };
                } else {
                    return Err(PageOutbound {
                        page: outline.page.get(),
                        offset: count,
                    });
                }

                if !outline.children.is_empty() {
                    offset_pages_rec(&mut outline.children, count)?;
                }
            }

            Ok(())
        }

        offset_pages_rec(&mut self.0, count)
    }
}

impl Toc<OutlineItem> {
    pub fn verify(&self) -> Result<(), TocError> {
        fn verify_rec(outlines: &[OutlineItem], pre_page: &mut usize) -> Result<(), TocError> {
            for outline in outlines {
                let Some(Destination::PageIndex(page)) = outline.dest else {
                    return Err(TocError::new(&outline.title, *pre_page));
                };

                if page < *pre_page {
                    return Err(TocError::new(&outline.title, *pre_page));
                }

                *pre_page = page;

                if !outline.children.is_empty() {
                    verify_rec(&outline.children, pre_page)?;
                }
            }

            Ok(())
        }

        verify_rec(&self.0, &mut 0)
    }

    pub fn offset_pages(&mut self, count: isize) {
        fn offset_pages_rec(outlines: &mut [OutlineItem], count: isize) {
            for outline in outlines {
                if let Some(Destination::PageIndex(page)) = outline.dest {
                    let new_page = page as isize + count;
                    // NOTE: mupdf-rs' page count from 0;
                    // if offset goes outbound, set the page to `None`
                    outline.dest =
                        (new_page >= 0).then_some(Destination::PageIndex(new_page as usize));
                }

                if !outline.children.is_empty() {
                    offset_pages_rec(&mut outline.children, count);
                }
            }
        }

        offset_pages_rec(&mut self.0, count)
    }
}
