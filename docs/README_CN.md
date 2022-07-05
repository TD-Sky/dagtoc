<h1 align="center">🐍 dagtoc 📖</h1>

<h5 align="center">一款用于添加/获取/删除pdf目录的命令行工具</h5>

> python v3.10.2+



## Introduction

**dagtoc**基于[PyMuPdf](https://github.com/pymupdf/PyMuPDF)，它是[MuPdf](https://mupdf.com/)的python绑定。Mupdf是一款历史悠久且功能强大的pdf解析器。



## Installation

**dagtoc**仅依赖于一个库：pymupdf。

```bash
$ pip3 install pymupdf
```

之后你就能直接使用`dagtoc.py`了。

如果你用[AUR](https://wiki.archlinux.org/title/Arch_User_Repository), 可以通过**AUR Helper**安装dagtoc。



## 演示

获取帮助

```bash
$ python dagtoc.py -h

usage: dagtoc.py [-h] [-d | -a TOC | -g | -c] [-r RMB] pdf

delete/add/get contents of pdf; contents information is carried by csv; line in csv: level(>0)|title|page number
删除/增添/获取pdf的目录; 目录信息记录在csv里; csv行格式：级别(>0)|标题|页码

positional arguments:
  pdf                   target pdf (目标pdf)

options:
  -h, --help            show this help message and exit
  -d, --delete          delete contents (删除目录)
  -a TOC, --add TOC     add contents (添加目录)
  -g, --get             get contents (获取目录)
  -c --check            check increasing (检查页码是否宽松单调递增)
  -r RMB, --revise RMB  RMB = Real page number — Book page number; it is used to correct offset of page numbers
                        RMB = 实际页码 — 书籍页码; 用于纠正页码偏移
```

接下来，我将演示如何使用dagtoc操作**demo**下的文件：

```bash
Makefile-NOTOC.pdf
Makefile.pdf
Makefile.toc
```

- 获取目录:

    ```bash
    $ python dagtoc.py -g Makefile.pdf -r 5
    ```

- 删除目录:

    ```bash
    $ python dagtoc.py -d Makefile.pdf
    ```

- 添加目录:

    **MuPdf**会覆写输入pdf的目录，输出一份新的pdf，输入pdf无变化。

    ```bash
    $ python dagtoc.py -a Makefile.toc -r 5 Makefile-NOTOC.pdf
    ```


- 检查页码是否宽松单调递增：

    ```bash
    $ python dagtoc.py -c Makefile.pdf
    ```



## 关于RMB参数

此参数属于选项`-r`。

**RMB**意即 *"实际页码减去书籍页码"*。

toc文件(.csv)内的页码应为**书籍页码**。导入TOC时，dagtoc令其加上**RMB**以得**实际页码**，为pdf阅读器所用。导出TOC时，dagtoc从实际页码减去**RMB**得到书籍页码。

**RMB**通常不为0，因为书籍的封面、前言、目录可能用罗马数字编号，正文则从1计起。因此你得将这些罗马数字换算为非正整数。



## 记住dagtoc

dagtoc = delete-add-get-TOC

