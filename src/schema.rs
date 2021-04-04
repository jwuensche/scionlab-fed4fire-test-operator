use serde::Deserialize;
use crate::experiment::run_on_host;
use anyhow::Result;

use std::collections::HashMap as Map;


#[derive(Deserialize)]
pub struct Config {
    pub hosts: Map<String, String>,
    pub network: NetHosts,
    pub experiments: Map<String, Experiment>,
}

#[derive(Deserialize)]
pub struct NetHosts {
    par: Option<Vec<Device>>,
    ams: Option<Vec<Device>>,
    gent: Option<Vec<Device>>,
}

impl NetHosts {
    pub fn iter(&self) -> NetHostsIterator {
        NetHostsIterator {
            hosts: &self,
            index: 0,
        }
    }
}

impl NetHosts {
    pub fn delete_all(&self, config: &Config) -> Result<()> {
        for (host, ips) in self.iter() {
            for dev in ips {
                run_on_host(&host, config, &format!("sudo ip route del {} dev {}", dev.ip, dev.dev))?;
            }
        }
        Ok(())
    }

    pub fn add_all(&self, config: &Config) -> Result<()> {
        for (host, ips) in self.iter() {
            for dev in ips {
                run_on_host(&host, config, &format!("sudo ip route add {} dev {}", dev.ip, dev.dev))?;
            }
        }
        Ok(())
    }
}

pub struct NetHostsIterator<'a> {
    hosts: &'a NetHosts,
    index: usize,
}

impl<'a> Iterator for NetHostsIterator<'a> {
    type Item = (String, &'a Vec<Device>);

    fn next(&mut self) -> Option<Self::Item> {
        let item;
        match self.index {
            0 => item = Some(("par".to_string(), self.hosts.par.as_ref())),
            1 => item = Some(("ams".to_string(), self.hosts.ams.as_ref())),
            2 => item = Some(("gent".to_string(), self.hosts.gent.as_ref())),
            _ => item = None,
        }
        self.index += 1;
        if let Some((_, None)) = item {
            return self.next()
        }
        item.map(|e| (e.0, e.1.unwrap()))
    }
}

#[derive(Deserialize)]
pub struct Device {
    pub dev: String,
    pub ip: String,
}

#[derive(Deserialize)]
pub struct Experiment {
    pub allow_ips: NetHosts,
    pub pre: Option<Treatment>,
    pub command: String,
    pub post: Option<Treatment>,
}

#[derive(Deserialize)]
pub struct Treatment {
    pub par: Option<String>,
    pub ams: Option<String>,
    pub gent: Option<String>,
}
