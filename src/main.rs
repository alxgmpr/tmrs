use std::process::Command;
use std::time::Instant;

use clap::Parser;
use env_logger::Builder;
use log::{LevelFilter, debug};
use statistics::mean;

fn std_deviation(data: &[f64]) -> Option<f64> {
    let count = data.len();
    if count < 2 {
        return None;
    }
    let data_mean = mean(data);
    let variance = data
        .iter()
        .map(|value| {
            let diff = data_mean - value;
            diff * diff
        })
        .sum::<f64>()
        / (count - 1) as f64;

    Some(variance.sqrt())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of runs to average
    #[arg(short, long, default_value = "5")]
    number: u32,

    /// Enable debug logging, log timing for each run
    #[arg(short, long)]
    debug: bool,

    /// Log the output of the ran commands to stdout
    #[arg(short, long)]
    verbose: bool,

    /// The command to time against
    #[arg(required = true)]
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
        let output = match Command::new(command).args(&command_args).output() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Failed to execute '{}': {}", command, e);
                std::process::exit(1);
            }
        };

        if !output.status.success() {
            eprintln!("Command '{}' failed with {}", command, output.status);
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
            println!("{}", String::from_utf8_lossy(&output.stdout));
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
        // Sample standard deviation of [1,2,3,4,5]: sqrt(2.5)
        let dev = result.unwrap();
        assert!((dev - 2.5_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_std_deviation_empty() {
        let data: Vec<f64> = vec![];
        assert!(std_deviation(&data).is_none());
    }

    #[test]
    fn test_std_deviation_single_value() {
        let data = vec![42.0];
        assert!(
            std_deviation(&data).is_none(),
            "Need at least 2 samples for sample standard deviation"
        );
    }

    #[test]
    fn test_std_deviation_identical_values() {
        let data = vec![3.0, 3.0, 3.0, 3.0];
        let result = std_deviation(&data).unwrap();
        assert!(
            result.abs() < 1e-10,
            "Identical values should have zero stddev"
        );
    }

    #[test]
    fn test_std_deviation_two_values() {
        let data = vec![0.0, 2.0];
        let result = std_deviation(&data).unwrap();
        // mean=1, diffs=[-1,1], sum_sq=2, variance=2/1=2, stddev=sqrt(2)
        assert!((result - std::f64::consts::SQRT_2).abs() < 1e-10);
    }

    #[test]
    fn test_std_deviation_known_dataset() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let result = std_deviation(&data).unwrap();
        // mean=5, sample variance = 32/7, sample stddev = sqrt(32/7)
        let expected = (32.0_f64 / 7.0).sqrt();
        assert!((result - expected).abs() < 1e-10);
    }
}
