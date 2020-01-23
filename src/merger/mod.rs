use slog::Logger;
use crate::toolbelt;
use std::path::PathBuf;
use toolbelt::logs::RootLog;
use serde_value::Value;

pub mod merge;

#[derive(Debug, Clone)]
pub struct SchemaMerger {
    input: PathBuf,
    mixin: PathBuf,
    log_conf: Logger,
}

impl SchemaMerger {

    pub fn new(files: (PathBuf, PathBuf)) -> Self {
        SchemaMerger {
            input: files.0,
            mixin: files.1,
            log_conf: RootLog::get_logger(false),
        }
    }

    pub fn set_logger(mut self, log_conf: Logger) -> Self {
        self.log_conf = log_conf;
        self
    }

    pub fn merge(&mut self)  {
        let a:Value = serde_value::Value::Bool(false);
        let b:Value = serde_value::Value::Bool(true);
        merge::MergeValue::new(a, b).merge();
    }

}
