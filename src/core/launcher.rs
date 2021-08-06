use std::process::Command;

use crate::{
    config::Config,
    errors::{EResult, EnaError},
};

pub struct VsCodeStatusCode(i32);

impl VsCodeStatusCode {
    pub fn is_ok(&self) -> bool {
        self.0 == 0
    }
}

pub trait Launcher {
    fn exec(&self, config: &Config, arguments: &impl ArgumentsGetters)
        -> EResult<VsCodeStatusCode>;
}

pub trait ArgumentsGetters {
    fn get_profile_name(&self) -> &str;
    fn get_path(&self) -> &str;
}

pub struct VsCodeLauncher;

// impl VsCodeLauncher {
//     fn get_
// }

impl Launcher for VsCodeLauncher {
    fn exec(
        &self,
        config: &Config,
        _arguments: &impl ArgumentsGetters,
    ) -> EResult<VsCodeStatusCode> {
        let vs_code_path = &config.vs_code_path;

        Err(EnaError::NotImplemented)
    }
}
