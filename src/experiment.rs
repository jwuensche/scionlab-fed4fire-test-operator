use crate::{Config, Experiment};
use anyhow::Result;

use std::process::Command;

const TESTMACHINE: &str = "testmachine";

impl Experiment {
    pub fn perform(&self, config: &Config) -> Result<()> {
        run_on_host(TESTMACHINE, config, &self.command)
    }

    pub fn allow_ips(&self, config: &Config) -> Result<()> {
        self.allow_ips.add_all(config)
    }

    pub fn prepare(&self, config: &Config) -> Result<()> {
        if let Some(pretreatment) = &self.pre {
            if let Some(par) = &pretreatment.par {
                run_on_host("par", config, par)?;
            }
            if let Some(ams) = &pretreatment.ams {
                run_on_host("ams", config, ams)?;
            }
            if let Some(gent) = &pretreatment.gent {
                run_on_host("gent", config, gent)?;
            }
        }
        Ok(())
    }

    pub fn post(&self, config: &Config) -> Result<()> {
        if let Some(posttreat) = &self.post {
            if let Some(par) = &posttreat.par {
                run_on_host("par", config, par)?;
            }
            if let Some(ams) = &posttreat.ams {
                run_on_host("ams", config, ams)?;
            }
            if let Some(gent) = &posttreat.gent {
                run_on_host("gent", config, gent)?;
            }
        }
        Ok(())
    }

    pub fn teardown(&self, config: &Config) -> Result<()> {
        self.allow_ips.delete_all(config)
    }
}

pub fn run_on_host(host: &str, config: &Config, command: &str) -> Result<()> {
    let connector = config.hosts.get(host).expect("Host not found");
    let result = Command::new("ssh")
        .args(connector.split_ascii_whitespace())
        .arg(command)
        .output()?;
    if !result.status.success() {
        println!("{}", String::from_utf8_lossy(&result.stderr));
        return Err(anyhow::Error::msg("return not succesful on host"))
    }
    Ok(())
}
