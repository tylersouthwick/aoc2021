use aoc2021::input::load_input;
use anyhow::Result;

fn main() -> Result<()> {
    let data : Vec<i64> = load_input(1)?;
    let depth_changes_part1 = find_depth_increases(&data, 1);
    println!("depth changes part1: {}", depth_changes_part1);

    let depth_changes_part2 = find_depth_increases(&data, 3);
    println!("depth changes part2: {}", depth_changes_part2);
    Ok(())
}

#[derive(Debug, PartialEq)]
enum DepthChange {
    NotApplicable,
    NoChange,
    Increased,
    Decreased,
}

fn create_windows(measurements : &Vec<i64>, window_size : usize) -> Vec<i64> {
    let mut windows = vec![];

    for i in 0..(measurements.len() - window_size + 1) {
        let sliding_window = &measurements.as_slice()[i..i+window_size];
        let mut sum = 0;
        for window in sliding_window.iter() {
            sum = sum + window;
        }
        windows.push(sum);
    }

    windows
}

fn calculate_depth_changes(measurements : &Vec<i64>, window_size : usize) -> Vec<DepthChange> {
    let mut previous_measurement : Option<i64> = None;
    let mut depth_changes = vec![];

    let windows = create_windows(measurements, window_size);
    for measurement in windows.iter() {
        depth_changes.push(match previous_measurement {
            Some(previous) => {
                if measurement == &previous {
                    DepthChange::NoChange
                } else if measurement > &previous {
                    DepthChange::Increased
                } else {
                    DepthChange::Decreased
                }
            },
            None => DepthChange::NotApplicable,
        });

        previous_measurement = Some(*measurement);
    }

    depth_changes
}

fn find_depth_increases(measurements : &Vec<i64>, window_size : usize) -> i64 {
    let depth_changes = calculate_depth_changes(measurements, window_size);

    let mut increase_count = 0;

    for depth_change in depth_changes.iter() {
        if depth_change == &DepthChange::Increased {
            increase_count = increase_count + 1;
        }
    }

    increase_count
}

#[test]
fn test_depth_increases_part1() {
    let depth_increase_count = find_depth_increases(&vec![
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263,
    ], 1);

    assert_eq!(depth_increase_count, 7);
}

#[test]
fn test_depth_increases_part2() {
    let depth_increase_count = find_depth_increases(&vec![
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263,
    ], 3);

    assert_eq!(depth_increase_count, 5);
}
