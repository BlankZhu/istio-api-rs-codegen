import os
import shutil
import pathlib
import re

import constant


def adjustRustCode(istio_version, section, api_version, crd):
    crd_dir = '/'.join([constant.generated_dir, istio_version,
                       section, api_version, crd])
    files = [f for f in os.listdir(
        crd_dir) if os.path.isfile(os.path.join(crd_dir, f))]

    istio_internal_resource_regex = 'istio_(.*?)_(v\d.*?\d)_(.*)\.rs'
    for f in files:
        match = re.search(istio_internal_resource_regex, f)
        if match:
            match_group = match.groups()
            matched_section = match_group[0]
            matched_api_version = match_group[1]
            matched_resource = match_group[2]

            if matched_section != section or matched_api_version != api_version:
                to_remove = os.path.join(crd_dir, f)
                print('removing', to_remove)
                os.remove(to_remove)
                continue

            # assume it's not a common resource type
            target_dir = os.path.join(
                constant.generated_dir, istio_version, matched_section, matched_api_version, matched_resource)
            if section in f and api_version in f:
                # it's a common resource type
                target_dir = os.path.join(
                    constant.generated_dir, istio_version, matched_section, matched_api_version, crd)
            if not os.path.exists(target_dir):
                pathlib.Path(target_dir).mkdir(parents=True, exist_ok=True)
            print('moving', os.path.join(crd_dir, f), 'to',
                  os.path.join(target_dir, matched_resource + ".rs"))
            shutil.move(os.path.join(crd_dir, f),
                        os.path.join(target_dir, matched_resource + ".rs"))
