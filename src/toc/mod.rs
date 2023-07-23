mod error;
pub use error::*;

use mupdf::Outline;

#[derive(Debug)]
pub struct Toc<S>(pub S);

#[derive(Debug)]
pub struct UserOutline<'toc> {
    pub title: &'toc str,
    pub page: u32,
    pub down: Vec<UserOutline<'toc>>,
}

pub type PymupdfOutline<'toc> = (u32, &'toc str, u32);

impl Toc<Vec<UserOutline<'_>>> {
    pub fn verify(&self) -> Result<(), TocError> {
        fn verify_rec(outlines: &[UserOutline<'_>], pre_page: &mut u32) -> Result<(), TocError> {
            for outline in outlines {
                if outline.page < *pre_page {
                    return Err(TocError::new(outline.title, *pre_page));
                }

                *pre_page = outline.page;

                if !outline.down.is_empty() {
                    verify_rec(&outline.down, pre_page)?;
                }
            }

            Ok(())
        }

        verify_rec(&self.0, &mut 0)
    }

    pub fn offset_pages(&mut self, count: i32) -> Result<(), PageOutbound> {
        fn offset_pages_internal(
            outlines: &mut [UserOutline<'_>],
            count: i32,
        ) -> Result<(), PageOutbound> {
            for outline in outlines {
                let page = outline.page as i32;

                let new_page = page + count;
                if new_page > 0 {
                    outline.page = new_page as u32;
                } else {
                    return Err(PageOutbound::new(outline.page, count));
                }

                if !outline.down.is_empty() {
                    offset_pages_internal(&mut outline.down, count)?;
                }
            }

            Ok(())
        }

        offset_pages_internal(&mut self.0, count)
    }

    pub fn as_pytoc(&self) -> Vec<PymupdfOutline<'_>> {
        fn as_pytoc_rec<'toc>(
            outlines: &'toc [UserOutline<'toc>],
            pytoc: &mut Vec<PymupdfOutline<'toc>>,
            level: u32,
        ) {
            for outline in outlines {
                pytoc.push((level, outline.title, outline.page));
                if !outline.down.is_empty() {
                    as_pytoc_rec(&outline.down, pytoc, level + 1);
                }
            }
        }

        let mut pytoc = vec![];
        as_pytoc_rec(&self.0, &mut pytoc, 1);
        pytoc
    }
}

impl Toc<Vec<Outline>> {
    pub fn verify(&self) -> Result<(), TocError> {
        fn verify_rec(outlines: &[Outline], pre_page: &mut u32) -> Result<(), TocError> {
            for outline in outlines {
                let page = outline
                    .page
                    .ok_or_else(|| TocError::new(&outline.title, *pre_page))?;

                if page < *pre_page {
                    return Err(TocError::new(&outline.title, *pre_page));
                }

                *pre_page = page;

                if !outline.down.is_empty() {
                    verify_rec(&outline.down, pre_page)?;
                }
            }

            Ok(())
        }

        verify_rec(&self.0, &mut 0)
    }

    pub fn offset_pages(&mut self, count: i32) {
        fn offset_pages_internal(outlines: &mut [Outline], count: i32) {
            for outline in outlines {
                if let Some(page) = outline.page.map(|p| p as i32) {
                    let new_page = page + count;
                    // NOTE: mupdf-rs' page count from 0;
                    // if offset goes outbound, set the page to `None`
                    outline.page = (new_page >= 0).then_some(new_page as u32);
                }

                if !outline.down.is_empty() {
                    offset_pages_internal(&mut outline.down, count);
                }
            }
        }

        offset_pages_internal(&mut self.0, count)
    }
}

impl AsRef<[Outline]> for Toc<Vec<Outline>> {
    #[inline]
    fn as_ref(&self) -> &[Outline] {
        &self.0
    }
}
