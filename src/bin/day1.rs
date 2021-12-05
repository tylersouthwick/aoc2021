use aoc2021::input::load_input;
use anyhow::Result;

fn main() -> Result<()> {
    let data : Vec<i64> = load_input(1)?;
    let depth_changes = find_depth_increases(data);
    println!("depth changes: {}", depth_changes);
    Ok(())
}

#[derive(Debug, PartialEq)]
enum DepthChange {
    NotApplicable,
    NoChange,
    Increased,
    Decreased,
}

fn calculate_depth_changes(measurements : Vec<i64>) -> Vec<DepthChange> {
    let mut previous_measurement : Option<i64> = None;
    let mut depth_changes = vec![];

    for measurement in measurements.iter() {
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

fn find_depth_increases(measurements : Vec<i64>) -> i64 {
    let depth_changes = calculate_depth_changes(measurements);

    let mut increase_count = 0;

    for depth_change in depth_changes.iter() {
        if depth_change == &DepthChange::Increased {
            increase_count = increase_count + 1;
        }
    }

    increase_count
}

#[test]
fn test_depth_increases() {
    let depth_increase_count = find_depth_increases(vec![
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
    ]);

    assert_eq!(depth_increase_count, 7);
}
