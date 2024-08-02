use std::path::PathBuf;

use directories::ProjectDirs;

use crate::system::mail_const::{APP_CACHE, APP_CONFIG, APP_DATA, APP_LOG, APP_NAME, ORG_DOMAIN, ORG_NAME};

#[derive(Debug, Clone)]
pub struct SysPath {
    pub app_path: PathBuf,
    pub app_data_path: PathBuf,
    pub app_config_path: PathBuf,
    pub app_log_path: PathBuf,
    pub app_cache_path: PathBuf,
    pub app_database_file: PathBuf,
}

pub fn app_path_initial() -> Option<SysPath> {
    if let Some(proj_dirs) = ProjectDirs::from(ORG_DOMAIN, ORG_NAME, APP_NAME) {
        let app_dir = proj_dirs.data_dir().to_path_buf();
        if app_dir.exists() == false {
            std::fs::create_dir_all(&app_dir).unwrap();
        }
        let app_data_path = app_dir.join(APP_DATA);
        if app_data_path.exists() == false {
            std::fs::create_dir_all(&app_data_path).unwrap();
        }
        let app_config_path = app_dir.join(APP_CONFIG);
        if app_config_path.exists() == false {
            std::fs::create_dir_all(&app_config_path).unwrap();
        }
        let app_log_path = app_dir.join(APP_LOG);
        if app_log_path.exists() == false {
            std::fs::create_dir_all(&app_log_path).unwrap();
        }
        let app_cache_path = app_dir.join(APP_CACHE);
        if app_cache_path.exists() == false {
            std::fs::create_dir_all(&app_cache_path).unwrap();
        }
        let app_database_file = app_data_path.join(crate::system::mail_const::APP_DATABASE_FILE);
        Some(SysPath {
            app_path: app_dir,
            app_data_path,
            app_config_path,
            app_log_path,
            app_cache_path,
            app_database_file,
        })
    } else {
        None
    }
}

impl SysPath {
    pub fn app_path(&self) -> &PathBuf {
        &self.app_path
    }
    pub fn app_data_path(&self) -> &PathBuf {
        &self.app_data_path
    }
    pub fn app_config_path(&self) -> &PathBuf {
        &self.app_config_path
    }
    pub fn app_log_path(&self) -> &PathBuf {
        &self.app_log_path
    }
    pub fn app_cache_path(&self) -> &PathBuf {
        &self.app_cache_path
    }
}
