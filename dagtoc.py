import argparse
import csv
import fitz


def delete_toc(in_pdf: str):
    """删除pdf目录"""
    doc = fitz.open(in_pdf)
    doc.set_toc(None)
    doc.save("NOTOC_{}".format(in_pdf))


def add_toc(in_pdf: str, toc_file: str, RMI: int):
    """导入csv为pdf的目录
    RMI: 真实页码减去输入页码之差
    """
    with open(toc_file, 'rt', newline='') as fp:
        toc = [ [ int(line[0]), line[1], int(line[2]) + RMI ] for line in csv.reader(fp, delimiter='|') ] 

    doc = fitz.open(in_pdf)
    doc.set_toc(toc)
    doc.save("TOC_{}".format(in_pdf))


def get_toc(in_pdf: str):
    """导出pdf的目录为csv"""
    doc = fitz.open(in_pdf)

    with open("{}.toc".format(in_pdf), 'w+', newline='') as fp:
        csv.writer(fp, delimiter='|').writerows(doc.get_toc())


def parse_args() -> list:
    """解析命令参数"""
    parser = argparse.ArgumentParser(description=
                                    "删除/增添/获取pdf的目录; "
                                    "目录导入/导出格式为csv; "
                                    "文件行: 目录级别|标题|页码")

    ex_group = parser.add_mutually_exclusive_group()
    ex_group.add_argument("-d", "--delete", action="store_true", help="删除目录")
    ex_group.add_argument("-a", "--add", dest="toc_file", type=str, default="", help="添加目录")
    ex_group.add_argument("-g", "--get", action="store_true", help="获取目录")
    parser.add_argument("-r", "--revise", dest="RMI", type=int, default=0,
                        help="修正所添加目录的页码误差: RMI = 实际值 - 输入值")
    parser.add_argument("pdf", type=str, help="文件(.pdf)")

    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()

    try:
        if args.delete == True:
            delete_toc(args.pdf)
        elif args.toc_file != "":
            add_toc(args.pdf, args.toc_file, args.RMI)
        elif args.get == True:
            get_toc(args.pdf)
        else:
            print("Incorrect operation")
    except Exception:
        print("Incorrect operation")

