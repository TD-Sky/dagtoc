<h1 align="center">🐍 dagtoc 📖</h1>

<h5 align="center">A command-line tool for adding, getting and deleting contents of pdf</h5>

## Introduction

**dagtoc** uses [MuPdf](https://mupdf.com/) as the underlying support.
At present dagtoc develops on both of bindings [PyMuPDF](https://github.com/pymupdf/PyMuPDF) and [mupdf-rs](https://github.com/messense/mupdf-rs).

I plan to replace PyMuPDF with mupdf-rs completely once mupdf-rs supports to add/delete outlines.



## Installation

### Language package managers

1. Install `pymupdf` via *pip*:

    ```bash
    $ pip3 install pymupdf
    ```

2. Install `dagtoc` via *cargo*:

   ```bash
   $ cargo install dagtoc
   ```

### Archlinux

- Install from source code:

  ```bash
  $ paru -S dagtoc
  ```

- Install binary:

  ```bash
  $ paru -S dagtoc-bin
  ```



## Usage

I will show how to use dagtoc with the files in [demo](./demo/) directory.

### TOC

The carrier format for TOC is [KDL](https://kdl.dev/) which is an excellent document language
for **forest structure**.

Outline looks like:

```
- <title> <page>
```

If it has suboutlines, they are written as the children of node:

```
- <title> <page> {
    - <title1> <page>
    - <title2> <page>
    ...
}
```

You can nest them certainly! See [Makefile.kdl](./demo/Makefile.kdl) for a real TOC.

### Add TOC

```bash
$ dagtoc -a Makefile.kdl Makefile-NOTOC.pdf -o Makefile.pdf
```

Additionally, you can use `-x` to specify the page number offset (+/-) for added TOC.
It would help you amend the TOC whose page numbers are all offset by the same constant.
**Page outbound error will occur if the number of any page is reduced to non-positive number.**

### Delete TOC

```bash
$ dagtoc -d Makefile.pdf -o Makefile-NOTOC.pdf
```

### Get TOC

```bash
$ dagtoc -g Makefile.pdf > Makefile.kdl
```

Additionally, you can use `-x` to specify the page number offset (+/-) for got TOC.
It would help you amend the TOC of pdf whose page numbers are all offset by the same constant.
**Page number will be empty if it is reduce to non-positive number.**

### Check TOC

```bash
$ dagtoc -c Makefile.pdf
```

Checking the page numbers are increasing and not empty in the pdf.



## Remember dagtoc

dagtoc = delete-add-get-TOC
