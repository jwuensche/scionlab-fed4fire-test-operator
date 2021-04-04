use crate::Config;

use anyhow::Result;

impl Config {
    pub fn prepare_network(&self) -> Result<()>{
        self.network.delete_all(&self)
    }

    pub fn reconstruct_network(&self) -> Result<()> {
        self.network.add_all(&self)
    }
}
