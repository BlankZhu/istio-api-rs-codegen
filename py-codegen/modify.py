import os
import re
from pathlib import Path

import constant
import util

normal_derive_regex = '#\[derive\(.*\)\]'
ref_find_regex = 'crate::models::\w*'
ref_search_regex = 'crate::models::(\w*)'
api_version_regex = '[vV]\d[a-zA-Z]*?\d'
mod_name_regex = '(\w*)\.rs'
uppercase_split_regex = '[A-Z][^A-Z]*'


def extractMetas(type_name):
    split_results = re.findall(uppercase_split_regex, type_name)
    section = split_results[1].lower()
    version = split_results[2].lower()
    type_name = ''.join(split_results[3:])
    if len(split_results) > 3:
        return [section, version, type_name]
    return []


def modifyNormalStruct(filename, istio_version, section, api_version, crd):
    crd_dir = os.path.join(constant.generated_dir,
                           istio_version, section, api_version, crd)
    new_type_name = filename[:len(filename) - 3].title().replace("_", "")
    old_type_name = 'Istio' + \
        section.title() + api_version[0].upper() + \
        api_version[1:] + new_type_name
    content = Path(os.path.join(crd_dir, filename)).read_text().strip()
    # rename the struct
    old_type_name_regex = '\s' + old_type_name + '\s'
    content = re.sub(old_type_name_regex, ' ' + new_type_name + ' ', content)
    # add schemars::JsonSchema
    content = 'use schemars::JsonSchema;\n' + content
    # add serde::{Deserialize, Serialize};
    content = 'use serde::{Deserialize, Serialize};\n' + content
    # add JsonSchema derive
    found = re.findall(normal_derive_regex, content)
    if found:
        old_derive = found[0]
        new_derive = old_derive[:len(old_derive) - 2] + ', JsonSchema)]'
        content = content.replace(old_derive, new_derive)
    else:
        print('derive regex not found!!')
    # handle possible reference
    refs = re.findall(ref_find_regex, content)
    if refs:
        for ref in refs:
            found = re.search(ref_search_regex, ref)
            type_name = found.group(1)

            metas = extractMetas(type_name)
            matched_section = metas[0]
            matched_version = metas[1]
            matched_type = metas[2]
            matched_type_phaton = '_'.join(
                [x.lower() for x in re.findall(uppercase_split_regex, matched_type)])

            # not in the same section/api_version/crd, use ref from crate
            if matched_section != section or matched_version != api_version:
                content = content.replace(
                    ref, "crate::" + matched_section + "::" + matched_version + "::" + matched_type_phaton + "::" + matched_type)
                continue

            # in the same section/api_version/crd
            api_version_section = re.findall(api_version_regex, type_name)
            if api_version_section:
                type_name = type_name[type_name.find(
                    api_version_section[0]) + len(api_version_section[0]):]
            print('new type name:', type_name,
                  'with found regex match:', found.group(1), 'api_version_section:', api_version_section)
            content = content.replace(ref, "super::" + type_name)

    with open(os.path.join(crd_dir, filename), "w") as f:
        f.write(content)
    return


def modifyCrdStruct(filename, istio_version, section, api_version, crd):
    crd_dir = os.path.join(constant.generated_dir,
                           istio_version, section, api_version, crd)
    new_type_base = filename[:len(filename) - 3].title().replace("_", "")
    old_type_name = 'Istio' + \
        section.title() + api_version[0].upper() + \
        api_version[1:] + new_type_base
    new_type_name = new_type_base + "Spec"
    content = Path(os.path.join(crd_dir, filename)).read_text().strip()
    # rename the struct
    old_type_name_regex = '\s' + old_type_name + '\s'
    content = re.sub(old_type_name_regex, ' ' + new_type_name + ' ', content)
    # add kube::CustomResource
    content = 'use kube::CustomResource;\n' + content
    # add schemars::JsonSchema
    content = 'use schemars::JsonSchema;\n' + content
    # add serde::{Deserialize, Serialize};
    content = 'use serde::{Deserialize, Serialize};\n' + content

    found = re.findall(normal_derive_regex, content)
    if found:
        # add JsonSchema derive & kube macro
        old_derive = found[0]
        new_derive = old_derive[:len(old_derive) - 2] + \
            ', JsonSchema, CustomResource)]'
        kube_derive = '#[kube(group = "' + section + ".istio.io" + '", version = "' + \
            api_version + '", kind = "' + new_type_base + '", namespaced)]'
        content = content.replace(old_derive, new_derive + '\n' + kube_derive)
    else:
        print('derive regex not found!!')
    # handle possible reference
    refs = re.findall(ref_find_regex, content)
    if refs:
        for ref in refs:
            found = re.search(ref_search_regex, ref)
            type_name = found.group(1)

            metas = extractMetas(type_name)
            matched_section = metas[0]
            matched_version = metas[1]
            matched_type = metas[2]
            matched_type_phaton = '_'.join(
                [x.lower() for x in re.findall(uppercase_split_regex, matched_type)])

            # not in the same section/api_version/crd, use ref from crate
            if matched_section != section or matched_version != api_version:
                if matched_section in constant.rust_reserved_words:
                    matched_section = 'r#' + matched_section
                content = content.replace(
                    ref, "crate::" + matched_section + "::" + matched_version + "::" + matched_type_phaton + "::" + matched_type)
                continue

            api_version_section = re.findall(api_version_regex, type_name)
            if api_version_section:
                type_name = type_name[type_name.find(
                    api_version_section[0]) + len(api_version_section[0]):]
            print('new type name:', type_name,
                  'with found regex match:', found.group(1))
            content = content.replace(ref, "super::" + type_name)

    with open(os.path.join(crd_dir, filename), "w") as f:
        f.write(content)
    return


def renameStruct(istio_version, section, api_version, crd):
    crd_dir = os.path.join(constant.generated_dir,
                           istio_version, section, api_version, crd)
    files = [f for f in os.listdir(
        crd_dir) if os.path.isfile(os.path.join(crd_dir, f))]
    for f in files:
        if f == 'mod.rs':
            continue
        if crd+'.rs' == f and not section in constant.section_with_no_crd:
            print('modifying CRD struct at:', crd+".rs")
            modifyCrdStruct(f, istio_version, section, api_version, crd)
            continue
        modifyNormalStruct(f, istio_version, section, api_version, crd)

    return


def remakeFinalModRs(istio_version, section, api_version, crd):
    crd_dir = os.path.join(constant.generated_dir, istio_version,
                           section, api_version, crd)
    mod_rs_file = os.path.join(crd_dir, 'mod.rs')
    content = []

    files = [f for f in os.listdir(
        crd_dir) if os.path.isfile(os.path.join(crd_dir, f)) and f != 'mod.rs']
    for f in files:
        mod_names = re.search(mod_name_regex, f)
        if mod_names:
            groups = mod_names.groups()
            mod_name = groups[0]
            content.append('pub mod ' + mod_name + ';')
            content.append('pub use self::' + mod_name + "::" +
                           mod_name.title().replace("_", "") + ";")
        else:
            print('cannot find mod name in', f, '!')

    with open(mod_rs_file, "w") as f:
        f.write('\n'.join(content))

    return


def MakeupModRsTree(istio_version):
    for dir in util.findAllDir(constant.generated_dir):
        fds = os.listdir(dir)
        if 'mod.rs' in fds:
            continue
        with open(os.path.join(dir, 'mod.rs'), "w") as mod_rs:
            for fd in fds:
                if fd.endswith('.rs'):
                    fd = fd[:len(fd) - 3]
                if fd in constant.rust_reserved_words:
                    fd = 'r#' + fd
                mod_rs.write("pub mod " + fd + ";\n")

    root_mod_rs = os.path.join(constant.generated_dir, istio_version, 'mod.rs')
    if os.path.exists(root_mod_rs):
        root_lib_rs = os.path.join(
            constant.generated_dir, istio_version, 'lib.rs')
        os.rename(root_mod_rs, root_lib_rs)
    else:
        print('cannot find root mod.rs!')

    return


def modifyRustCode(istio_version, section, api_version, crd):
    # rename struct
    renameStruct(istio_version, section, api_version, crd)

    # remake final mod.rs
    remakeFinalModRs(istio_version, section, api_version, crd)

    return
