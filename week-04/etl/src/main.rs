// ETL Example

#[derive(Debug)]
struct RawData {
    id: u32,
    value: i32,
}

#[derive(Debug)]
struct CleanData {
    id: u32,
    value: i32,
}

fn main() {
    let raw = vec![
        RawData { id: 1, value: 10 },
        RawData { id: 2, value: -5 },
        RawData { id: 3, value: 42 },
        RawData { id: 4, value: 71 },
        RawData { id: 5, value: 100 },
        RawData { id: 6, value: 113 },
        RawData { id: 7, value: 25 },
        RawData { id: 8, value: 51 },
    ];

    let cleaned = extract_transform_load(raw);

    for item in &cleaned {
        println!("Clean Data: Id - {:?} Value - {:?}", item.id, item.value); // Accessing the fields
    }
}

// Perform ETL process
fn extract_transform_load(raw: Vec<RawData>) -> Vec<CleanData> {
    raw.into_iter()
        .map(|r| CleanData {
            id: r.id,
            value: r.value.max(0),
        })
        .collect()
}
