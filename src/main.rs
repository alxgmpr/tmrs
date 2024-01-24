use std::process::Command;
use std::str;
use std::time::Instant;

use clap::Parser;
use env_logger::Builder;
use log::{debug, LevelFilter};
use statistics::mean;

fn std_deviation(data: &Vec<f64>) -> Option<f32> {
    let data_mean = mean(data);
    let count = data.len();
    if count > 0 {
        let variance = data
            .iter()
            .map(|value| {
                let diff = data_mean - *value;
                diff * diff
            })
            .sum::<f64>()
            / count as f64;

        Some(variance.sqrt() as f32)
    } else {
        None
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of runs to average
    #[clap(short, long, default_value = "5")]
    number: u32,

    /// Enable debug logging, log timing for each run
    #[clap(short, long)]
    debug: bool,

    /// Log the output of the ran commands to stdout
    #[clap(short, long)]
    verbose: bool,

    /// The command to time against
    #[clap(required = true)]
    command: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let log_level = if args.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    Builder::new().filter(None, log_level).init();

    let num_runs: u32 = args.number;
    debug!("Running {} times", num_runs);

    let command = &args.command[0];
    let command_args: Vec<&str> = args.command[1..].iter().map(AsRef::as_ref).collect();

    debug!("Command: {}", command);
    debug!("Command args: {:?}", command_args);

    let mut times = Vec::new();
    for index in 0..num_runs {
        let start = Instant::now();
        let output = Command::new(command)
            .args(command_args.clone())
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("Command failed: {}", output.status);
            std::process::exit(1);
        }

        let duration = start.elapsed();
        times.push(duration.as_secs_f64());

        debug!(
            "Run #{} completed in {:.3} seconds",
            index + 1,
            duration.as_secs_f64()
        );

        if args.verbose && !output.stdout.is_empty() {
            debug!("Command output:");
            println!("{}", str::from_utf8(&output.stdout).unwrap());
        }
    }
    debug!("All runs completed, calculating mean and standard deviation");

    let avg_time = mean(&times);
    let std_dev = std_deviation(&times);

    debug!("Calculated mean: {}", avg_time);

    match std_dev {
        Some(dev) => debug!("Calculated standard deviation: {}", dev),
        None => debug!("Calculated standard deviation: N/A"),
    }

    println!("avg: {:.3}sec", avg_time);
    match std_dev {
        Some(dev) => println!("std dev: {:.3}sec", dev),
        None => println!("std dev: N/A"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_deviation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = std_deviation(&data);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), std::f32::consts::SQRT_2);
    }
}
