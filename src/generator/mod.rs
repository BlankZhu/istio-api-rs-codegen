use crate::constant;
use crate::error;
use crate::r#type;
use crate::utility;
use log::warn;
use log::{debug, info};
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

        let mut mod_names = Vec::new();
        for rd in read_dir {
            match rd {
                Ok(rd) => match rd.file_type() {
                    Ok(ft) => {
                        if ft.is_dir() {
                            self.read_api_group_dir(rd.path().as_path(), &rd.file_name());
                        }

                        if let Ok(mod_name_string) = rd.file_name().into_string() {
                            mod_names.push(mod_name_string);
                        } else {
                            warn!("cannot convert {:?} to String", rd.file_name());
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

        let out_dir = path::Path::new(constant::ISTIO_CRD_RUST_CODE_OUTPUT_DIRECTORY);
        if let Err(e) = self.make_lib_rs(out_dir, mod_names) {
            error!(
                "failed to generate `lib.rs` on {:?}/mod.rs: {}",
                out_dir.display(),
                e
            );
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

        let mut mod_names = Vec::new();
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

                            if let Ok(mod_name_string) = rd.file_name().into_string() {
                                mod_names.push(mod_name_string);
                            } else {
                                warn!("cannot convert {:?} to String", rd.file_name());
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

        let out_istio_ver_dir =
            path::Path::new(constant::ISTIO_CRD_RUST_CODE_OUTPUT_DIRECTORY).join(istio_ver);
        if let Err(e) = self.make_mod_rs(out_istio_ver_dir.as_path(), mod_names) {
            error!(
                "failed to generate `mod.rs` on {:?}/mod.rs: {}",
                out_istio_ver_dir.display(),
                e
            );
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

        let mut mod_names = Vec::new();
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

                            if let Ok(mod_name_string) = rd.file_name().into_string() {
                                mod_names.push(mod_name_string);
                            } else {
                                warn!("cannot convert {:?} to String", rd.file_name());
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

        let out_api_group_dir = path::Path::new(constant::ISTIO_CRD_RUST_CODE_OUTPUT_DIRECTORY)
            .join(istio_ver)
            .join(api_group);
        if let Err(e) = self.make_mod_rs(out_api_group_dir.as_path(), mod_names) {
            error!(
                "failed to generate `mod.rs` on {:?}/mod.rs: {}",
                out_api_group_dir.display(),
                e
            );
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

        let mut mod_names = Vec::new();
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

                                if let Ok(mod_name_string) = stem.to_os_string().into_string() {
                                    mod_names.push(mod_name_string);
                                } else {
                                    warn!("cannot convert {:?} to String", stem);
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

        let out_api_ver_dir = path::Path::new(constant::ISTIO_CRD_RUST_CODE_OUTPUT_DIRECTORY)
            .join(istio_ver)
            .join(api_group)
            .join(api_ver);
        if let Err(e) = self.make_mod_rs(out_api_ver_dir.as_path(), mod_names) {
            error!(
                "failed to generate `mod.rs` on {:?}/mod.rs: {}",
                out_api_ver_dir.display(),
                e
            );
        }
    }

    fn make_yaml_file_path(
        &self,
        istio_ver: &OsStr,
        api_group: &OsStr,
        api_ver: &OsStr,
        kind: &OsStr,
    ) -> r#type::Result<path::PathBuf> {
        let base_dir = path::Path::new(constant::ISTIO_CRD_TEMP_DIRECTORY);
        self.make_final_path(base_dir, istio_ver, api_group, api_ver, kind, "yaml")
    }

    fn make_rs_file_path(
        &self,
        istio_ver: &OsStr,
        api_group: &OsStr,
        api_ver: &OsStr,
        kind: &OsStr,
    ) -> r#type::Result<path::PathBuf> {
        let base_dir = path::Path::new(constant::ISTIO_CRD_RUST_CODE_OUTPUT_DIRECTORY);
        self.make_final_path(base_dir, istio_ver, api_group, api_ver, kind, "rs")
    }

    fn make_final_path(
        &self,
        base_dir: &path::Path,
        istio_ver: &OsStr,
        api_group: &OsStr,
        api_ver: &OsStr,
        kind: &OsStr,
        extension: &str,
    ) -> r#type::Result<path::PathBuf> {
        let file_dir_path = path::Path::new(base_dir)
            .join(istio_ver)
            .join(api_group)
            .join(api_ver);
        if !file_dir_path.exists() {
            fs::create_dir_all(&file_dir_path)?;
        }
        if kind.to_str().is_none() {
            let err_msg = format!("failed to get raw string from kind OsStr: {:?}", kind);
            return Err(Box::new(error::KindError::new(err_msg.as_str())));
        }
        let final_filename = utility::camel_to_snake(kind.to_str().unwrap());
        let final_file_path = file_dir_path.join(final_filename).with_extension(extension);
        Ok(final_file_path)
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

        let rs_file_path = self.make_rs_file_path(istio_ver, api_group, api_ver, kind)?;
        let yaml_file_path = self.make_yaml_file_path(istio_ver, api_group, api_ver, kind)?;

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

    fn make_mod_rs(&self, curr_dir: &path::Path, mut mod_names: Vec<String>) -> r#type::Result<()> {
        let mut content = String::new();
        mod_names.sort();
        for name in mod_names {
            let line = format!("pub mod {};\n", name);
            content += &line;
        }
        let mod_rs_path = curr_dir.join("mod.rs");
        if !content.is_empty() {
            fs::write(mod_rs_path, content)?;
        }
        Ok(())
    }

    fn make_lib_rs(&self, curr_dir: &path::Path, mut mod_names: Vec<String>) -> r#type::Result<()> {
        let mut content = String::new();
        mod_names.sort();
        for name in mod_names {
            let ver_without_patch = utility::extract_major_minor_version(name.as_str());
            let mod_line = format!(
                "#[cfg(feature = \"{}\")] mod {};\n",
                ver_without_patch, ver_without_patch
            );
            let use_line = format!(
                "#[cfg(feature = \"{}\")] pub use self::{}::*;\n\n",
                ver_without_patch, name
            );
            content += &mod_line;
            content += &use_line;
        }
        let mod_rs_path = curr_dir.join("lib.rs");
        if !content.is_empty() {
            fs::write(mod_rs_path, content)?;
        }
        Ok(())
    }
}
