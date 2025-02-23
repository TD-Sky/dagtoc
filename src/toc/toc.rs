use std::num::NonZeroU32;

use mupdf::Outline;

use super::{PageOutbound, TocError};

#[derive(Debug)]
pub struct Toc<O>(pub(crate) Vec<O>);

#[derive(Debug)]
pub struct UserOutline<'toc> {
    pub title: &'toc str,
    pub page: NonZeroU32,
    pub down: Vec<UserOutline<'toc>>,
}

impl From<Toc<UserOutline<'_>>> for Vec<Outline> {
    fn from(user: Toc<UserOutline>) -> Self {
        user.0.iter().map(Outline::from).collect()
    }
}

impl From<&UserOutline<'_>> for Outline {
    fn from(user: &UserOutline) -> Self {
        Self {
            title: user.title.to_owned(),
            uri: None,
            page: Some(user.page.get() - 1),
            down: user.down.iter().map(|item| item.into()).collect(),
            x: f32::NAN,
            y: f32::NAN,
        }
    }
}

impl Toc<UserOutline<'_>> {
    pub fn verify(&self) -> Result<(), TocError> {
        fn verify_rec(outlines: &[UserOutline<'_>], pre_page: &mut u32) -> Result<(), TocError> {
            for outline in outlines {
                if outline.page.get() < *pre_page {
                    return Err(TocError::new(outline.title, *pre_page));
                }

                *pre_page = outline.page.get();

                if !outline.down.is_empty() {
                    verify_rec(&outline.down, pre_page)?;
                }
            }

            Ok(())
        }

        verify_rec(&self.0, &mut 0)
    }

    pub fn offset_pages(&mut self, count: i32) -> Result<(), PageOutbound> {
        fn offset_pages_rec(
            outlines: &mut [UserOutline<'_>],
            count: i32,
        ) -> Result<(), PageOutbound> {
            for outline in outlines {
                let page = outline.page.get() as i32;

                let new_page = page + count;
                if new_page > 0 {
                    outline.page = unsafe { NonZeroU32::new_unchecked(new_page as u32) };
                } else {
                    return Err(PageOutbound::new(outline.page.get(), count));
                }

                if !outline.down.is_empty() {
                    offset_pages_rec(&mut outline.down, count)?;
                }
            }

            Ok(())
        }

        offset_pages_rec(&mut self.0, count)
    }
}

impl Toc<Outline> {
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
        fn offset_pages_rec(outlines: &mut [Outline], count: i32) {
            for outline in outlines {
                if let Some(page) = outline.page.map(|p| p as i32) {
                    let new_page = page + count;
                    // NOTE: mupdf-rs' page count from 0;
                    // if offset goes outbound, set the page to `None`
                    outline.page = (new_page >= 0).then_some(new_page as u32);
                }

                if !outline.down.is_empty() {
                    offset_pages_rec(&mut outline.down, count);
                }
            }
        }

        offset_pages_rec(&mut self.0, count)
    }
}
