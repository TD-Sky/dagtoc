[TOC]

### 介绍

dagtoc是基于 PyMuPdf 的命令行软件, 它可以让你轻松使用 MuPdf 对pdf的目录进行简单的操作(增加/获取/移除).

[MuPdf](https://mupdf.com/) 是一款历史悠久而且强大的pdf解析器, 而[PyMuPdf](https://github.com/pymupdf/PyMuPDF) 是它的Python包装, dagtoc所做的工作是为 PyMuPdf 制作了一个命令行前端.



### 安装

首先, 请使用 pip 安装`pymupdf`:

```shell
$ pip install pymupdf
```

此库是dagtoc唯一的依赖, 之后直接使用`dagtoc.py`即可.



### 演示

以 demo 目录下的文件为例:

```shell
$ ls demo/
Makefile.pdf
Makefile.pdf.toc
```

获取目录:

```shell
$ python dagtoc.py -g Makefile.pdf
```

移除目录:

```shell
$ python dagtoc.py -d Makefile.pdf
```

增加目录:

虽然 MuPdf 创建目录的操作是覆盖式的, 但我还是建议使用无目录的pdf进行此项操作!

```shell
$ python dagtoc.py -a Makefile.pdf.toc -r 0 NOTOC_Makefile.pdf
```

选项`-r`指定了页码误差参数, 即`真实页码(阅读器显示页码) - 书籍页码`, 用于修正页码的偏移. 许多书籍的扉页、前言、目录部分用的是罗马数字标记, 你需要手动换算为负数.



### 记住dagtoc

dagtoc = delete-add-get-TOC
