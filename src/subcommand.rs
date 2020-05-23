use crate::config::Config;

pub struct RepositoryCmd<'a> {
    config: &'a Config,
}

impl<'a> RepositoryCmd<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}
