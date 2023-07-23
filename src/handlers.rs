use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use kdl::KdlDocument;
use mupdf::Document;
use pyo3::types::PyTuple;
use pyo3::Python;

use crate::codec::IntoKdlDocument;
use crate::codec::TryAsOutlines;
use crate::toc::Toc;

pub fn delete(pdf: &Path, output: &Path) -> Result<()> {
    Python::with_gil(|py| {
        let fitz = py.import("fitz")?;
        let pdf = fitz.getattr("open")?.call1(PyTuple::new(py, [pdf]))?;

        let args = PyTuple::new(py, [py.None()]);
        pdf.call_method1("set_toc", args)?;

        let args = PyTuple::new(py, [output]);
        pdf.call_method1("save", args)?;

        pdf.call_method0("close")?;

        Ok(())
    })
}

pub fn get(pdf: &Path, offset: i32) -> Result<()> {
    let pdf = Document::open(pdf.to_string_lossy().as_ref())?;
    let mut toc = Toc(pdf.outlines()?);

    if offset != 0 {
        toc.offset_pages(offset);
    }

    let toc = toc.as_ref().into_kdl_doc();
    print!("{toc}");

    Ok(())
}

pub fn add(pdf: &Path, output: &Path, toc: &Path, offset: i32) -> Result<()> {
    let mut buf = String::new();
    File::open(toc)?.read_to_string(&mut buf)?;
    let toc: KdlDocument = buf.parse()?;

    let mut toc = Toc(toc.try_as_outlines()?);
    toc.verify()?;

    if offset != 0 {
        toc.offset_pages(offset)?;
    }

    Python::with_gil(|py| {
        let fitz = py.import("fitz")?;
        let pdf = fitz.getattr("open")?.call1(PyTuple::new(py, [pdf]))?;

        let args = PyTuple::new(py, [toc.as_pytoc()]);
        pdf.call_method1("set_toc", args)?;

        let args = PyTuple::new(py, [output]);
        pdf.call_method1("save", args)?;

        pdf.call_method0("close")?;

        Ok(())
    })
}

pub fn check(pdf: &Path) -> Result<()> {
    let pdf = Document::open(pdf.to_string_lossy().as_ref())?;
    let toc = Toc(pdf.outlines()?);
    toc.verify()?;

    Ok(())
}
