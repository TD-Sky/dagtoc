#!/usr/bin/python

import argparse
import csv
import fitz


def delete_toc(in_pdf: str):
    """删除pdf目录"""
    doc = fitz.open(in_pdf)
    doc.set_toc(None)
    doc.save("{}-no-toc".format(in_pdf))


def add_toc(in_pdf: str, toc_file: str, RMB: int):
    """导入csv为pdf的目录"""
    with open(toc_file, 'rt', newline='') as fp:
        toc = [ [ int(line[0]), line[1], int(line[2]) + RMB ] for line in csv.reader(fp, delimiter='|') ] 

    doc = fitz.open(in_pdf)
    doc.set_toc(toc)
    doc.save("{}-with-toc".format(in_pdf))


def get_toc(in_pdf: str, RMB: int):
    """导出pdf的目录为csv"""
    doc = fitz.open(in_pdf)

    toc = doc.get_toc()
    for t in toc:
        t[2] -= RMB

    with open("{}.toc".format(in_pdf), 'w+', newline='') as fp:
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
        if args.delete == True:
            delete_toc(args.pdf)
        elif args.toc != "":
            add_toc(args.pdf, args.toc, args.RMB)
        elif args.get == True:
            get_toc(args.pdf, args.RMB)
        else:
            print("Incorrect operation")
    except Exception as exp:
        #raise exp
        print("Incorrect operation")

