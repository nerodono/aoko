use std::net::IpAddr;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
    /// IP address of the switch
    pub ip: IpAddr,

    /// Port of the sys-hidplus server
    #[clap(long, short, default_value_t = 8000)]
    pub port: u16,

    /// How much packets will be sent in one second.
    ///
    /// Theoretically this is the latency adjust
    #[clap(long, short, default_value_t = 128.0)]
    pub ticks: f64,

    /// Stick axis multiplier. Used to convert input from [-1; 1] to integral value
    /// with the following formula: `[input * multiplier]`
    #[clap(long, short, default_value_t = 32767.0)]
    pub axis_multiplier: f32,

    /// Invert X axis of the left stick
    #[clap(long, short)]
    pub invert_left_x: bool,

    /// Invert Y axis of the left stick
    #[clap(long, short)]
    pub invert_left_y: bool,

    /// Invert X axis of the right stick
    #[clap(long, short)]
    pub invert_right_x: bool,

    /// Invert Y axis of the right stick
    #[clap(long, short)]
    pub invert_right_y: bool,
}

impl CliArgs {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
