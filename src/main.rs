mod schema;
mod experiment;
mod preparation;

use std::{fs::OpenOptions, time::Duration};
use anyhow::Result;
use schema::*;

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("test.yml")?;
    let config: Config = serde_yaml::from_reader(file)?;
    config.prepare_network()?;
    for (exp_name, exp) in config.experiments.iter() {
        println!("Performing network experiment {}!", exp_name);
        exp.allow_ips(&config)?;
        exp.prepare(&config)?;
        exp.perform(&config)?;
        exp.post(&config)?;
        exp.teardown(&config)?;
        std::thread::sleep(Duration::from_millis(500));
    }
    config.reconstruct_network()?;
    Ok(())
}
