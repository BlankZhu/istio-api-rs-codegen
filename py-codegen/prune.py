import shutil
import os

import constant


def pruneRustCode(istio_version, section, api_version, crd):
    crd_dir = '/'.join([constant.generated_dir, istio_version,
                       section, api_version, crd])
    crd_models_dir = '/'.join([crd_dir, 'src', 'models'])

    shutil.rmtree('/'.join([crd_dir, '.openapi-generator']))
    shutil.rmtree('/'.join([crd_dir, 'docs']))
    shutil.rmtree('/'.join([crd_dir, 'src', 'apis']))

    # clear crd_dir
    files = [f for f in os.listdir(
        crd_dir) if os.path.isfile(os.path.join(crd_dir, f))]
    for f in files:
        os.remove(os.path.join(crd_dir, f))

    # clear src_dir
    files = [f for f in os.listdir(
        os.path.join(crd_dir, 'src')) if os.path.isfile(os.path.join(crd_dir, 'src', f))]
    for f in files:
        os.remove(os.path.join(crd_dir, 'src', f))

    # move all in crd/src/models/ to crd/
    files = [f for f in os.listdir(crd_models_dir) if os.path.isfile(
        os.path.join(crd_models_dir, f))]
    for f in files:
        shutil.move(os.path.join(crd_models_dir, f), os.path.join(crd_dir, f))
    shutil.rmtree('/'.join([crd_dir, 'src']))
