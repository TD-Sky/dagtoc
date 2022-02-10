#!/usr/bin/python3

import argparse
import csv
import fitz


def delete_toc(in_pdf: str):
    """删除pdf目录"""
    doc = fitz.open(in_pdf)
    doc.set_toc(None)
    out_pdf: str = in_pdf.rsplit('.', 1)[0] + "-NOTOC.pdf"
    doc.save(out_pdf)


def add_toc(in_pdf: str, toc_file: str, RMB: int):
    """导入csv为pdf的目录"""
    with open(toc_file, 'rt', encoding="utf-8", newline='') as fp:
        toc = [ ( int(line[0]), line[1], int(line[2]) + RMB ) for line in csv.reader(fp, delimiter='|') ] 

    verify_increasing(toc)

    doc = fitz.open(in_pdf)
    doc.set_toc(toc)
    out_pdf: str = in_pdf.rsplit('.', 1)[0] + "-TOC.pdf"
    doc.save(out_pdf)


def verify_increasing(toc: list):
    """确保页码排列是宽松单调递增的"""
    for i in range(0, len(toc) - 1):
        if toc[i][2] > toc[i+1][2]:
            raise RuntimeError("Wrong page occurs in line {} or {} of TOC.".format(i+1, i+2))


def get_toc(in_pdf: str, RMB: int):
    """导出pdf的目录为csv"""
    doc = fitz.open(in_pdf)

    toc = doc.get_toc()
    for t in toc:
        t[2] -= RMB

    toc_file: str = in_pdf.rsplit('.', 1)[0] + ".toc"
    with open(toc_file, 'w+', encoding="utf-8", newline='') as fp:
        csv.writer(fp, delimiter='|').writerows(toc)


def parse_args() -> list:
    """解析命令参数"""
    parser = argparse.ArgumentParser(description=
                                    "删除/增添/获取pdf的目录; "
                                    "目录导入/导出格式为csv; "
                                    "文件行: 目录级别|标题|页码")

    ex_group = parser.add_mutually_exclusive_group()
    ex_group.add_argument("-d", "--delete", action="store_true", help="删除目录")
    ex_group.add_argument("-a", "--add", dest="toc", type=str, default="", help="添加目录")
    ex_group.add_argument("-g", "--get", action="store_true", help="获取目录")
    parser.add_argument("-r", "--revise", dest="RMB", type=int, default=0,
                        help="RMB = 实际页码 — 书籍页码; 用于修正csv内的页码误差, 默认为0")
    parser.add_argument("pdf", type=str, help="文件(.pdf)")

    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()

    try:
        if args.delete:
            delete_toc(args.pdf)
        elif args.toc != "":
            add_toc(args.pdf, args.toc, args.RMB)
        elif args.get:
            get_toc(args.pdf, args.RMB)
        else:
            print("Unknown operation!")
    except UnicodeDecodeError:
        print("Failed: {} isn't encoded in UTF-8.".format(args.toc))

