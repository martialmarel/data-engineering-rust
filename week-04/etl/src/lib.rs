#[derive(Debug)]
pub struct RawData {
    pub id: u32,
    pub value: i32,
}

#[derive(Debug)]
pub struct CleanData {
    pub id: u32,
    pub value: i32,
}

// Perform ETL process
pub fn extract_transform_load(raw: Vec<RawData>) -> Vec<CleanData> {
    raw.into_iter()
        .map(|r| CleanData {
            id: r.id,
            value: r.value.max(0),
        })
        .collect()
}
