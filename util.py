import os


def findAllFile(base):
    for root, ds, fs in os.walk(base):
        for f in fs:
            fullname = os.path.join(root, f)
            yield fullname


def findAllDir(base):
    for root, ds, fs in os.walk(base):
        for d in ds:
            dirname = os.path.join(root, d)
            yield dirname


def removeSuffix(input_string, suffix):
    if suffix and input_string.endswith(suffix):
        return input_string[:-len(suffix)]
    return input_string


def generateMetadatas(base):
    ret = list()
    for path in findAllFile(base):
        split = path.split('/')
        if len(split) == 5:
            split[4] = removeSuffix(split[4], '.gen.json')
            ret.append(split[1:])
    return ret
