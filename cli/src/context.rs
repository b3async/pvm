use crate::error::ContextError;
use crate::{pvm_build_path, pvm_path, pvm_versions_path};

use lenaris::{SysInfo, Vendor};
use std::fs::create_dir_all;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Context {
    build_path: PathBuf,
    version_path: PathBuf,
    vendor: Vendor,
}

impl Context {
    fn init_folders(&self) -> Result<(), std::io::Error> {
        if !self.version_path.exists() {
            create_dir_all(&self.version_path)?;
        }

        if !self.build_path.exists() {
            create_dir_all(&self.build_path)?;
        }

        Ok(())
    }

    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.init_folders()?;

        Ok(())
    }

    pub fn vendor(&self) -> Vendor {
        self.vendor.clone()
    }
}

pub struct ContextBuilder {
    build_path: Option<PathBuf>,
    version_path: Option<PathBuf>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            build_path: None,
            version_path: None,
        }
    }

    pub fn build_path(&mut self, path: PathBuf) -> &mut Self {
        self.build_path = Some(path);

        self
    }

    pub fn version_path(&mut self, path: PathBuf) -> &mut Self {
        self.version_path = Some(path);

        self
    }

    pub fn build(&self) -> Result<Context, ContextError> {
        let build_path = match self.build_path.as_ref() {
            Some(path) => path.clone(),
            None => return Err(ContextError::MissingProperty("build path".to_owned())),
        };

        let version_path = match self.version_path.as_ref() {
            Some(path) => path.clone(),
            None => return Err(ContextError::MissingProperty("version path".to_owned())),
        };

        let context = Context {
            build_path,
            version_path,
            vendor: Vendor::discover::<SysInfo>().expect("unable to build vendor"),
        };

        Ok(context)
    }
}

impl Default for Context {
    fn default() -> Self {
        ContextBuilder::new()
            .build_path(pvm_build_path!())
            .version_path(pvm_versions_path!())
            .build()
            .expect("unable to build app context")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_context() {
        let _ = ContextBuilder::new()
            .build_path(pvm_build_path!())
            .version_path(pvm_versions_path!())
            .build()
            .unwrap();

        assert!(ContextBuilder::new().build().is_err());

        assert!(ContextBuilder::new()
            .build_path(pvm_build_path!())
            .build()
            .is_err());
    }

    #[test]
    fn can_build_context_with_defaults() {
        let _ = Context::default();
    }
}
