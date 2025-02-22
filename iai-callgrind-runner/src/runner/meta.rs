use std::ffi::OsString;
use std::path::PathBuf;

use log::debug;

use crate::error::Result;
use crate::runner::envs;
use crate::util::get_absolute_path;

#[derive(Debug, Clone)]
pub struct Cmd {
    pub bin: PathBuf,
    pub args: Vec<OsString>,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub arch: String,
    pub aslr_enabled: bool,
    pub target_dir: PathBuf,
    pub valgrind: Cmd,
    pub valgrind_wrapper: Option<Cmd>,
}

impl Metadata {
    pub fn new() -> Result<Self> {
        let arch = std::env::consts::ARCH.to_owned();
        debug!("Detected architecture: {}", arch);

        let target_dir = std::env::var_os(envs::CARGO_TARGET_DIR)
            .map_or_else(
                || {
                    cargo_metadata::MetadataCommand::new()
                        .no_deps()
                        .exec()
                        .map_or_else(
                            |_| PathBuf::from("target"),
                            |p| p.target_directory.into_std_path_buf(),
                        )
                },
                PathBuf::from,
            )
            .join("iai")
            .join(std::env::var_os(envs::CARGO_PKG_NAME).map_or_else(PathBuf::new, PathBuf::from));
        debug!("Detected target directory: '{}'", target_dir.display());

        let aslr_enabled = std::env::var_os(envs::IAI_CALLGRIND_ALLOW_ASLR).is_some();
        if aslr_enabled {
            debug!(
                "Found {} environment variable. Running with ASLR enabled.",
                envs::IAI_CALLGRIND_ALLOW_ASLR
            );
        }

        // Invoke Valgrind, disabling ASLR if possible because ASLR could noise up the results a bit
        let valgrind_path = get_absolute_path("valgrind")?;
        let valgrind_wrapper = if aslr_enabled {
            debug!("Running with ASLR enabled");
            None
        } else if cfg!(target_os = "linux") {
            debug!("Trying to run with ASLR disabled: Using 'setarch'");

            if let Ok(set_arch) = get_absolute_path("setarch") {
                Some(Cmd {
                    bin: set_arch,
                    args: vec![
                        OsString::from(&arch),
                        OsString::from("-R"),
                        OsString::from(&valgrind_path),
                    ],
                })
            } else {
                debug!("Failed to switch ASLR off: 'setarch' not found. Running with ASLR enabled");
                None
            }
        } else if cfg!(target_os = "freebsd") {
            debug!("Trying to run with ASLR disabled: Using 'proccontrol'");

            if let Ok(proc_control) = get_absolute_path("proccontrol") {
                Some(Cmd {
                    bin: proc_control,
                    args: vec![
                        OsString::from("-m"),
                        OsString::from("aslr"),
                        OsString::from("-s"),
                        OsString::from("disable"),
                        OsString::from(&valgrind_path),
                    ],
                })
            } else {
                debug!(
                    " Failed to switch ASLR off: 'proccontrol' not found. Running with ASLR \
                     enabled"
                );
                None
            }
        } else {
            debug!("Failed to switch ASLR off. No utility available. Running with ASLR enabled");
            None
        };

        Ok(Self {
            arch,
            aslr_enabled,
            target_dir,
            valgrind: Cmd {
                bin: valgrind_path,
                args: vec![],
            },
            valgrind_wrapper,
        })
    }
}
