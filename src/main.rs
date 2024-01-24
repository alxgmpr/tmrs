use std::env;
use std::process::Command;
use std::time::Instant;

use getopts::Options;
use statistics::mean;

fn std_deviation(data: &Vec<f64>) -> Option<f32> {
    let data_mean = mean(data);
    let count = data.len();
    if count > 0 {
        let variance = data.iter().map(|value| {
            let diff = data_mean - *value;
            diff * diff
        }).sum::<f64>() / count as f64;

        Some(variance.sqrt() as f32)
    } else {
        None
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("n", "number", "set number of runs", "NUM");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };

    let num_runs: u32 = matches.opt_str("n").and_then(|s| s.parse().ok()).unwrap_or(5);
    let command = if !matches.free.is_empty() { &matches.free[0] } else { panic!("No command provided"); };
    let command_args = if matches.free.len() > 1 { &matches.free[1..] } else { &[] };

    let mut times = Vec::new();
    for _ in 0..num_runs {
        let start = Instant::now();
        let output = Command::new(command)
            .args(command_args)
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("Command failed: {}", output.status);
            std::process::exit(1);
        }

        let duration = start.elapsed();
        times.push(duration.as_secs_f64());
    }
    let avg_time = mean(&times);
    let std_dev = std_deviation(&times);

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