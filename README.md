## 介绍

dagtoc是 PyMuPdf 的命令行前端, 它能让你轻松**增添/获取/移除** pdf 的目录.

[PyMuPdf](https://github.com/pymupdf/PyMuPDF) 是 [MuPdf](https://mupdf.com/) 的 Python 绑定, 而 MuPdf 是一款历史悠久且功能强悍的pdf解析器.



## 安装

首先, 请使用 pip 安装`pymupdf`:

```shell
$ pip install pymupdf
```

此库是dagtoc唯一的依赖, 之后直接使用`dagtoc.py`即可.



## 演示

以 demo 目录下的文件为例:

```shell
$ ls demo/
Makefile.pdf
Makefile.pdf.toc
Makefile.pdf-no-toc
```

获取目录:

```shell
$ python dagtoc.py -g Makefile.pdf -r 5
```

移除目录:

```shell
$ python dagtoc.py -d Makefile.pdf
```

增加目录:

虽然 MuPdf 创建目录的操作是覆盖式的, 但我还是建议使用无目录的pdf进行此项操作!

```shell
$ python dagtoc.py -a Makefile.pdf.toc -r 5 Makefile.pdf-no-toc
```



## 参数 RMB

此参数由选项`-r`指定, 形式为`-r RMB`.

RMB: Real page number Minus Book page number. (真实页码 - 书籍页码)

csv文件内的页码应是**书籍页码**. 导入时我们令其加上**RMB**以得**真实页码**(阅读器显示页码); 导出时我们让真实页码减去**RMB**以得书籍页码.

很多时候**RMB**都不为0, 因为书籍的封面、前言、目录部分可能使用罗马数字标记, 尔后在内容部分自1数起. 对此, 你必须手动换算罗马数字页码至非正整数.



## 记住dagtoc

dagtoc = delete-add-get-TOC
