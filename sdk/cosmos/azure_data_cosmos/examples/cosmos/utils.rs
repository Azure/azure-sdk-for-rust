use azure_data_cosmos::models::ThroughputProperties;
use clap::Args;

#[derive(Args, Clone)]
pub struct ThroughputOptions {
    /// Enables autoscaling and sets the maximum RUs to support. Cannot be used if `--manual` is set.
    #[clap(long)]
    auto_scale: Option<usize>,

    /// Sets the increment percentage for autoscale. Ignored unless `--auto-scale` is set.
    #[clap(long)]
    auto_scale_increment: Option<usize>,

    /// Provisions manual throughput, specifying the number of RUs.
    #[clap(long)]
    manual: Option<usize>,
}

impl TryFrom<ThroughputOptions> for Option<ThroughputProperties> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(v: ThroughputOptions) -> Result<Self, Box<dyn std::error::Error>> {
        match (v.auto_scale, v.manual) {
            (Some(_), Some(_)) => Err("cannot set both '--auto-scale' and '--manual'".into()),
            (Some(max), None) => Ok(Some(ThroughputProperties::auto_scale(
                max,
                v.auto_scale_increment,
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
        opt.ok_or("must specify either '--auto-scale' or '--manual'".into())
    }
}
