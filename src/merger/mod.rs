use slog::Logger;
use crate::toolbelt;
use std::path::PathBuf;
use toolbelt::logs::RootLog;
use serde_value::Value;

pub mod merge;

#[derive(Debug, Clone)]
pub struct SchemaMerger {
    file_one: PathBuf,
    file_two: PathBuf,
    log_conf: Logger,
}

impl SchemaMerger {

    pub fn new(files: (PathBuf, PathBuf)) -> Self {
        SchemaMerger {
            file_one: files.0,
            file_two: files.1,
            log_conf: RootLog::get_logger(false),
        }
    }

    pub fn set_logger(mut self, log_conf: Logger) -> Self {
        self.log_conf = log_conf;
        self
    }

    pub fn merge(&mut self, a: Value, b: Value)  {
        merge::MergeValue::new(a, b).merge();
    }

}
