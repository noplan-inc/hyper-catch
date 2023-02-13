use clap::Parser;
use formatter::OutputFormatter;
use option::{CliOptions, NetworkConfig};
use std::fs;
use time::UtcOffset;
use tracing_subscriber::fmt::time::OffsetTime;

mod contract;
mod contract_getter;
mod formatter;
mod nft_types;
mod option;
mod outputter;
mod safe_ethers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliOptions::parse();

    //todo put subscriber into a new another function, but this didn't work https://www.reddit.com/r/rust/comments/uqsfmw/is_there_a_rule_that_tracing_subscriberfmtinit/
    let offset = UtcOffset::from_hms(9, 0, 0).expect("should get JST offset");
    let time_format =
        time::format_description::parse("[year]-[month]-[day] T [hour]:[minute]:[second]")
            .expect("failed to time offset");
    let file_appender = tracing_appender::rolling::hourly("./", "ethers_log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    if args.verbose {
        tracing_subscriber::fmt()
            .with_timer(OffsetTime::new(offset, time_format))
            .with_writer(non_blocking)
            .with_ansi(false)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_timer(OffsetTime::new(offset, time_format))
            .with_writer(non_blocking)
            .with_max_level(tracing::Level::ERROR)
            .with_ansi(false)
            .init();
    }

    let config_string: String =
        fs::read_to_string(args.config).expect("failed to load config file");
    let setting: NetworkConfig = toml::from_str(&config_string).expect("failed to parse toml");

    let mut getter = contract_getter::NftContractGetter::new(&setting.rpc, setting.name)
        .await
        .expect("Invalid RPC endpoint");
    let mut outputter =
        outputter::Outputter::new(&setting.output).expect("failed to create outputter");

    for block_number in setting.from..setting.to + 1 {
        let found_contracts = getter.find(block_number).await;

        if found_contracts.len() != 0 {
            output(&mut outputter, found_contracts, &setting.format)?;
        }
    }
    print!("done");
    Ok(())
}

fn output(
    outputter: &mut outputter::Outputter,
    contracts: Vec<contract::Contract>,
    output_format: &option::OutputFormat,
) -> anyhow::Result<()> {
    match output_format {
        option::OutputFormat::Json => {
            let formatter = formatter::Json::new();
            for found_contract in contracts {
                outputter.write(formatter.format(&found_contract)?)?;
            }
        }
        option::OutputFormat::Csv => {
            let formatter = formatter::Csv::new();
            outputter.write(formatter.header()?)?;
            for found_contract in contracts {
                outputter.write(formatter.format(&found_contract)?)?;
            }
        }
    }

    Ok(())
}
