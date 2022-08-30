use crate::r#type;
use log::{debug, error, info};
use std::ffi::OsStr;
use std::fs;
use std::path;

#[derive(Debug)]
pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        return Generator {};
    }

    pub fn generate(&self, istio_crd_temp_directory: &str) -> r#type::Result<()> {
        let path = path::Path::new(istio_crd_temp_directory);
        self.read_istio_version_dir(path);
        Ok(())
    }

    fn read_istio_version_dir(&self, tmp_dir: &path::Path) {
        debug!("reading istio tmp dir: {:?}", tmp_dir);
        let read_dir = match fs::read_dir(tmp_dir) {
            Ok(rd) => rd,
            Err(e) => {
                error!("failed to read dir {:?}: {}", tmp_dir, e);
                return;
            }
        };

        for rd in read_dir {
            match rd {
                Ok(rd) => match rd.file_type() {
                    Ok(ft) => {
                        if ft.is_dir() {
                            self.read_api_group_dir(rd.path().as_path(), &rd.file_name());
                        }
                    }
                    Err(e) => {
                        error!("failed to read file type of: {}", e);
                        continue;
                    }
                },
                Err(e) => {
                    error!("failed to read dir entry: {}", e);
                    continue;
                }
            }
        }
    }

    fn read_api_group_dir(&self, istio_ver_dir: &path::Path, istio_ver: &OsStr) {
        debug!("reading istio version dir: {:?}", istio_ver_dir);
        let read_dir = match fs::read_dir(istio_ver_dir) {
            Ok(rd) => rd,
            Err(e) => {
                error!("failed to read dir {:?}: {}", istio_ver_dir, e);
                return;
            }
        };

        for rd in read_dir {
            match rd {
                Ok(rd) => match rd.file_type() {
                    Ok(ft) => {
                        if ft.is_dir() {
                            self.read_api_version_dir(
                                rd.path().as_path(),
                                istio_ver,
                                &rd.file_name(),
                            );
                        }
                    }
                    Err(e) => {
                        error!("failed to read file type of: {}", e);
                        continue;
                    }
                },
                Err(e) => {
                    error!("failed to read dir entry: {}", e);
                    continue;
                }
            }
        }
    }

    fn read_api_version_dir(
        &self,
        api_group_dir: &path::Path,
        istio_ver: &OsStr,
        api_group: &OsStr,
    ) {
        debug!("reading api group dir: {:?}", api_group_dir);
        let read_dir = match fs::read_dir(api_group_dir) {
            Ok(rd) => rd,
            Err(e) => {
                error!("failed to read dir {:?}: {}", api_group_dir, e);
                return;
            }
        };

        for rd in read_dir {
            match rd {
                Ok(rd) => match rd.file_type() {
                    Ok(ft) => {
                        if ft.is_dir() {
                            self.read_kinds(
                                rd.path().as_path(),
                                istio_ver,
                                api_group,
                                &rd.file_name(),
                            );
                        }
                    }
                    Err(e) => {
                        error!("failed to read file type of: {}", e);
                        continue;
                    }
                },
                Err(e) => {
                    error!("failed to read dir entry: {}", e);
                    continue;
                }
            }
        }
    }

    fn read_kinds(
        &self,
        api_ver_dir: &path::Path,
        istio_ver: &OsStr,
        api_group: &OsStr,
        api_ver: &OsStr,
    ) {
        debug!("reading api version dir: {:?}", api_ver_dir);
        let read_dir = match fs::read_dir(api_ver_dir) {
            Ok(rd) => rd,
            Err(e) => {
                error!("failed to read dir {:?}: {}", api_ver_dir, e);
                return;
            }
        };

        for rd in read_dir {
            match rd {
                Ok(rd) => match rd.file_type() {
                    Ok(ft) => {
                        if ft.is_file() {
                            if let Some(stem) = rd.path().file_stem() {
                                if let Err(e) =
                                    self.call_kopium(istio_ver, api_group, api_ver, stem)
                                {
                                    error!("failed to call kopium: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("failed to read file type of: {}", e);
                        continue;
                    }
                },
                Err(e) => {
                    error!("failed to read dir entry: {}", e);
                    continue;
                }
            }
        }
    }

    fn call_kopium(
        &self,
        istio_ver: &OsStr,
        api_group: &OsStr,
        api_ver: &OsStr,
        kind: &OsStr,
    ) -> r#type::Result<()> {
        info!(
            "calling kopium with istio_ver: {}, api_group: {}, api_ver: {}, kind: {}",
            istio_ver.to_str().unwrap(),
            api_group.to_str().unwrap(),
            api_ver.to_str().unwrap(),
            kind.to_str().unwrap()
        );
        Ok(())
    }
}
