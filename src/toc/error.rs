#[derive(Debug)]
pub struct TocError {
    pub title: String,
    pub pre_page: usize,
}

#[derive(Debug)]
pub struct PageOutbound {
    pub page: usize,
    pub offset: isize,
}

impl TocError {
    pub fn new(title: impl Into<String>, pre_page: usize) -> Self {
        Self {
            title: title.into(),
            pre_page,
        }
    }
}

impl std::fmt::Display for TocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error occurs in TOC: title={}, previous page={}",
            self.title, self.pre_page
        )
    }
}

impl std::error::Error for TocError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for PageOutbound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "page offset went out bound: page={}, offset={}",
            self.page, self.offset
        )
    }
}

impl std::error::Error for PageOutbound {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
