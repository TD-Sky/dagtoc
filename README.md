<h1 align="center">🐍 dagtoc 📖</h1>

<h5 align="center">A command-line tool for adding, getting and deleting contents of pdf</h5>

<p align="center">
  <a href="docs/README_CN.md">简体中文</a>
</p>

> python v3.10.2+



## Introduction

**dagtoc** is based on [PyMuPdf](https://github.com/pymupdf/PyMuPDF) that is the python binding of [MuPdf](https://mupdf.com/). Mupdf is a celebrated and powerful pdf parser.



## Installation

**dagtoc** only depends on one library: pymupdf.

```bash
$ pip3 install pymupdf
```

After then you are able to use `dagtoc.py` directly.

If you use [AUR](https://wiki.archlinux.org/title/Arch_User_Repository), you could install dagtoc from there via **AUR Helper**.



## Demostration

Ask for help:

```bash
$ python dagtoc.py -h

usage: dagtoc.py [-h] [-d | -a TOC | -g | -c] [-r RMB] pdf

delete/add/get contents of pdf; contents information is carried by csv;
line in csv: level(>0)|title|page number

positional arguments:
  pdf                   target pdf

options:
  -h, --help            show this help message and exit
  -d, --delete          delete contents
  -a TOC, --add TOC     add contents
  -g, --get             get contents
  -c, --check           check increasing
  -r RMB, --revise RMB  RMB = Real page number — Book page number; it is used
                        to correct offset of page numbers
```

Now, I will show how to use dagtoc by operating the files in directory **demo**:

```bash
Makefile-NOTOC.pdf
Makefile.pdf
Makefile.toc
```

- Getting contents:

    ```bash
    $ python dagtoc.py -g Makefile.pdf -r 5
    ```

- Deleting contents:

    ```bash
    $ python dagtoc.py -d Makefile.pdf
    ```

- Adding contents:

    **MuPdf** will output a new pdf by overwriting the original contents of input pdf. The input pdf will not change.

    ```bash
    $ python dagtoc.py -a Makefile.toc -r 5 Makefile-NOTOC.pdf
    ```

- Checking the increasing of pages:

    ```bash
    $ python dagtoc.py -c Makefile.pdf
    ```



## About argument RMB

This argument belongs to option `-r`.

**RMB** means  *"Real page number Minus Book page number"*.

The pages in toc file(.csv) should be **Book page number**. When importing TOC, dagtoc adds **RMB** to it to get **Real page number** used by pdf reader. When exporting TOC, dagtoc subtracts **RMB** from Real page number to get Book page number.

**RMB** is usually not 0, because cover, preface and contents of book may be numbered in Roman numerals. Then main body are counted from 1.Therefore you have to covert these Roman numerals to nonpositive integers.



## Remember dagtoc

dagtoc = delete-add-get-TOC

