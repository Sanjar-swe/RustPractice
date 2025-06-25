use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Weather CLI")]
#[command(about = "Get weather info via OpenWeatherMap", long_about = None)]

pub struct Args {
    #[arg(short = 'c', long)]
    pub city: String,

    #[arg(short = 'C', long)]
    pub country: String,

    #[arg(short = 'F', long, default_value = "human")]
    pub format:String,

    #[arg(long)]
    pub api_key: Option<String>,
}

#[test]
    fn test_parse_args_valid(){
        let result = Args::try_parse_from([
            "test-bin",
            "--city", "Tashkent",
            "--country", "UZ",
            "--format", "json",
        ]);
        assert!(result.is_ok());
        
        let args = result.expect("Args should parse correctly");
        assert_eq!(args.city, "Tashkent");
        assert_eq!(args.country, "UZ");
        assert_eq!(args.format, "json");
}
