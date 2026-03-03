// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::models::ThroughputProperties;
use clap::Args;

#[derive(Args, Clone)]
pub struct ThroughputOptions {
    /// Enables autoscaling and sets the maximum RUs to support. Cannot be used if `--manual` is set.
    #[arg(long)]
    autoscale: Option<usize>,

    /// Sets the increment percentage for autoscale. Ignored unless `--autoscale` is set.
    #[arg(long)]
    autoscale_increment: Option<usize>,

    /// Provisions manual throughput, specifying the number of RUs.
    #[arg(long)]
    manual: Option<usize>,
}

impl TryFrom<ThroughputOptions> for Option<ThroughputProperties> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(v: ThroughputOptions) -> Result<Self, Box<dyn std::error::Error>> {
        match (v.autoscale, v.manual) {
            (Some(_), Some(_)) => Err("cannot set both '--autoscale' and '--manual'".into()),
            (Some(max), None) => Ok(Some(ThroughputProperties::autoscale(
                max,
                v.autoscale_increment,
            ))),
            (None, Some(rus)) => Ok(Some(ThroughputProperties::manual(rus))),
            (None, None) => Ok(None),
        }
    }
}

impl TryFrom<ThroughputOptions> for ThroughputProperties {
    type Error = Box<dyn std::error::Error>;

    fn try_from(v: ThroughputOptions) -> Result<Self, Box<dyn std::error::Error>> {
        let opt: Option<ThroughputProperties> = v.try_into()?;
        opt.ok_or("must specify either '--autoscale' or '--manual'".into())
    }
}

pub fn print_throughput(throughput: ThroughputProperties) {
    if let Some(tp) = throughput.throughput() {
        println!("  Throughput: {}RU/s", tp);
    } else {
        println!("  Throughput: Unlimited");
    }
    if let Some(autoscale_max) = throughput.autoscale_maximum() {
        println!("  Autoscale max: {}RU/s", autoscale_max);
    }
    if let Some(autoscale_incr) = throughput.autoscale_increment() {
        println!("  Autoscale increment: {}%", autoscale_incr);
    }
}
