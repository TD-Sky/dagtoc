use std::fs;

use kdl::KdlDocument;
use pdf_oxide::{
    OutlineItem, PdfDocument,
    editor::{DocumentEditor, EditableDocument},
};

use crate::{
    cli::{AddArgs, CheckArgs, DeleteArgs, GetArgs},
    toc::Toc,
};

pub fn delete(DeleteArgs { pdf, output }: &DeleteArgs) -> eros::Result<()> {
    let mut pdf = DocumentEditor::open(pdf)?;
    // pdf.delete_outlines()?;
    pdf.save(output)?;
    Ok(())
}

pub fn get(GetArgs { pdf, offset }: &GetArgs) -> eros::Result<()> {
    let pdf = PdfDocument::open(pdf)?;
    let mut toc = Toc(pdf.get_outline()?.unwrap_or_default());

    if *offset != 0 {
        toc.offset_pages(*offset);
    }

    let toc = KdlDocument::from(&toc);
    print!("{toc}");

    Ok(())
}

pub fn add(
    AddArgs {
        toc,
        pdf,
        output,
        offset,
    }: &AddArgs,
) -> eros::Result<()> {
    let buf = fs::read_to_string(toc)?;
    let toc: KdlDocument = buf.parse()?;
    let mut toc = Toc::try_from(&toc)?;

    toc.verify()?;

    if *offset != 0 {
        toc.offset_pages(*offset)?;
    }

    let toc: Vec<OutlineItem> = toc.into();

    let mut pdf = DocumentEditor::open(pdf)?;
    // pdf.set_outlines(&toc)?;
    pdf.save(output)?;

    Ok(())
}

pub fn check(CheckArgs { pdf }: &CheckArgs) -> eros::Result<()> {
    let pdf = PdfDocument::open(pdf)?;
    let toc = Toc(pdf.get_outline()?.unwrap_or_default());
    toc.verify()?;

    Ok(())
}
