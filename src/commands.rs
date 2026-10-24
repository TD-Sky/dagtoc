use std::fs;
use std::path::Path;

use kdl::KdlDocument;
use mupdf::pdf::PdfDocument;
use mupdf::{Document, Outline};

use crate::cli::{AddArgs, CheckArgs, DeleteArgs, GetArgs};
use crate::toc::Toc;

pub fn delete(DeleteArgs { pdf, output }: &DeleteArgs) -> anyhow::Result<()> {
    let mut pdf = PdfDocument::open(pdf.to_string_lossy().as_ref())?;
    pdf.delete_outlines()?;
    pdf.save(output.to_string_lossy().as_ref())?;
    Ok(())
}

pub fn get(GetArgs { pdf, offset }: &GetArgs) -> anyhow::Result<()> {
    let pdf = Document::open(pdf.to_string_lossy().as_ref())?;
    let mut toc = Toc(pdf.outlines()?);

    if offset != 0 {
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
) -> anyhow::Result<()> {
    let buf = fs::read_to_string(toc)?;
    let toc: KdlDocument = buf.parse()?;
    let mut toc = Toc::try_from(&toc)?;

    toc.verify()?;

    if offset != 0 {
        toc.offset_pages(*offset)?;
    }

    let toc: Vec<Outline> = toc.into();

    let mut pdf = PdfDocument::open(pdf.to_string_lossy().as_ref())?;
    pdf.set_outlines(&toc)?;
    pdf.save(output.to_string_lossy().as_ref())?;

    Ok(())
}

pub fn check(CheckArgs { pdf }: &CheckArgs) -> anyhow::Result<()> {
    let pdf = Document::open(pdf.to_string_lossy().as_ref())?;
    let toc = Toc(pdf.outlines()?);
    toc.verify()?;

    Ok(())
}
