use std::fmt;

#[derive(Debug)]
pub struct TocError {
    title: String,
    pre_page: u32,
}

#[derive(Debug)]
pub struct PageOutbound {
    page: u32,
    offset: i32,
}

impl TocError {
    #[inline]
    pub fn new(title: impl Into<String>, pre_page: u32) -> Self {
        Self {
            title: title.into(),
            pre_page,
        }
    }
}

impl fmt::Display for TocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error occurs in TOC: title={}, previous page={}",
            self.title, self.pre_page
        )
    }
}

impl std::error::Error for TocError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl PageOutbound {
    #[inline]
    pub fn new(page: u32, offset: i32) -> Self {
        Self { page, offset }
    }
}

impl fmt::Display for PageOutbound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "page offset went out bound: page={}, offset={}",
            self.page, self.offset
        )
    }
}

impl std::error::Error for PageOutbound {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
