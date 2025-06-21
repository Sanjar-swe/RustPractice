use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Weather CLI")]
#[command(about = "Get weather info via OpenWeatherMap", long_about = None)]

pub struct Args {
    #[arg(short, long)]
    pub city: String,

    #[arg(short, long)]
    pub country: String,

    #[arg(short = 'F', long, default_value = "human")]
    pub format:String,

    #[arg(long)]
    pub api_key: Option<String>,
}

