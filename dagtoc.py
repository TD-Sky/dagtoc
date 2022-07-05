#!/usr/bin/python3

import argparse
import csv
import fitz
from pathlib import Path


def delete_toc(in_pdf: str):
    """删除pdf目录"""
    out_pdf: str = Path(in_pdf).stem + "-NOTOC.pdf"

    with fitz.open(in_pdf) as doc:
        doc.set_toc(None)
        doc.save(out_pdf)


def verify_increasing(toc: list):
    """确保页码排列宽松单调递增"""
    for i in range(0, len(toc) - 1):
        if toc[i][2] > toc[i+1][2]:
            raise RuntimeError("Wrong page occurs at line {} or {} in TOC".format(i+1, i+2))


def add_toc(in_pdf: str, toc_file: str, RMB: int):
    """导入csv为pdf的目录"""
    out_pdf: str = Path(in_pdf).stem + "-TOC.pdf"

    with open(toc_file, 'rt', encoding="utf-8", newline='') as fp:
        toc = [ ( int(line[0]), line[1], int(line[2]) + RMB ) for line in csv.reader(fp, delimiter='|') ]

    verify_increasing(toc)

    with fitz.open(in_pdf) as doc:
        doc.set_toc(toc)
        doc.save(out_pdf)


def check_toc(in_pdf: str):
    """检查目录页码是否单调递增"""
    with fitz.open(in_pdf) as doc:
        toc = doc.get_toc()

    verify_increasing(toc)


def get_toc(in_pdf: str, RMB: int):
    """导出pdf的目录为csv"""
    toc_file: str = Path(in_pdf).stem + ".toc"

    with fitz.open(in_pdf) as doc:
        toc = doc.get_toc()
        for t in toc:
            t[2] -= RMB

    with open(toc_file, 'w+', encoding="utf-8", newline='') as fp:
        csv.writer(fp, delimiter='|').writerows(toc)


def parse_args() -> list:
    """解析命令参数"""
    parser = argparse.ArgumentParser(description=
                                     "delete/add/get contents of pdf; "
                                     "contents information is carried by csv; "
                                     "line in csv: level(>0)|title|page number")

    ex_group = parser.add_mutually_exclusive_group()
    ex_group.add_argument("-d", "--delete", action="store_true", help="delete contents")
    ex_group.add_argument("-a", "--add", dest="toc", type=str, default="", help="add contents")
    ex_group.add_argument("-g", "--get", action="store_true", help="get contents")
    ex_group.add_argument("-c", "--check", action="store_true", help="check increasing")
    parser.add_argument("-r", "--revise", dest="RMB", type=int, default=0,
                        help="RMB = Real page number — Book page number; it is used to correct offset of page numbers")
    parser.add_argument("pdf", type=str, help="target pdf")

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
        elif args.check:
            check_toc(args.pdf)
        else:
            print("[ERROR] Undefined operation")
    except UnicodeDecodeError:
        print("[ERROR] {} isn't encoded in UTF-8".format(args.toc))
