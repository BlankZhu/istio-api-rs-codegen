use crate::constant;
use crate::r#type;
use crate::utility;
use log::{debug, error, info};
use std::ffi::OsStr;
use std::fs;
use std::path;
use std::process;

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

        let rs_dir_path = path::Path::new(constant::ISTIO_CRD_RUST_CODE_OUTPUT_DIRECTORY)
            .join(istio_ver)
            .join(api_group)
            .join(api_ver);
        if !rs_dir_path.exists() {
            fs::create_dir_all(&rs_dir_path)?;
        }
        if kind.to_str().is_none() {
            error!("failed to get raw string from kind OsStr: {:?}", kind);
            return Ok(());
        }
        let rs_filename = utility::CamelToSnake(kind.to_str().unwrap());
        let rs_file_path = rs_dir_path.join(rs_filename).with_extension("rs");

        let yaml_dir_path = path::Path::new(constant::ISTIO_CRD_TEMP_DIRECTORY)
            .join(istio_ver)
            .join(api_group)
            .join(api_ver);
        let yaml_filename = path::Path::new(kind).with_extension("yaml");
        let yaml_file_path = yaml_dir_path.join(yaml_filename);

        info!(
            "calling kopium to generate rust code from `{}` to `{}`",
            yaml_file_path.display(),
            rs_file_path.display()
        );

        let output = process::Command::new(constant::KOPIUM_COMMAND)
            .args(["-Af", yaml_file_path.to_str().unwrap()])
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        fs::write(rs_file_path, stdout.as_ref())?;

        Ok(())
    }
}
