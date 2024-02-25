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
        .map(|r| {
            let value = match r.value {
                v if v < 0 => 0,
                v if v > 100 => 100,
                _ => r.value,
            };

            CleanData { id: r.id, value }
        })
        .collect()
}

pub struct Summary {
    pub total: i32,
    pub average: f64,
}

pub fn summarize(cleaned: &Vec<CleanData>) -> Summary {
    let total = cleaned.iter().fold(0, |acc, x| acc + x.value);
    let count = cleaned.len() as i32;
    let average = total as f64 / count as f64;

    Summary { total, average }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn have_same_count_input_output() {
        let raw = vec![
            RawData { id: 1, value: 11 },
            RawData { id: 2, value: 22 },
            RawData { id: 3, value: 33 },
        ];

        let cleaned = extract_transform_load(raw);

        assert_eq!(cleaned.len(), 3);
        assert_eq!(cleaned[0].id, 1);
        assert_eq!(cleaned[0].value, 11);
        assert_eq!(cleaned[1].id, 2);
        assert_eq!(cleaned[1].value, 22);
        assert_eq!(cleaned[2].id, 3);
        assert_eq!(cleaned[2].value, 33);
    }

    #[test]
    fn a_negative_number_set_to_zero() {
        let raw = vec![
            RawData { id: 1, value: -11 },
            RawData { id: 2, value: 22 },
            RawData { id: 3, value: -33 },
        ];

        let cleaned = extract_transform_load(raw);

        assert_eq!(cleaned.len(), 3);
        assert_eq!(cleaned[0].id, 1);
        assert_eq!(cleaned[0].value, 0);
        assert_eq!(cleaned[1].id, 2);
        assert_eq!(cleaned[1].value, 22);
        assert_eq!(cleaned[2].id, 3);
        assert_eq!(cleaned[2].value, 0);
    }

    #[test]
    fn a_number_over_hundred_set_to_hundred() {
        let raw = vec![
            RawData { id: 1, value: 101 },
            RawData { id: 2, value: 22 },
            RawData { id: 3, value: 133 },
        ];

        let cleaned = extract_transform_load(raw);

        assert_eq!(cleaned.len(), 3);
        assert_eq!(cleaned[0].id, 1);
        assert_eq!(cleaned[0].value, 100);
        assert_eq!(cleaned[1].id, 2);
        assert_eq!(cleaned[1].value, 22);
        assert_eq!(cleaned[2].id, 3);
        assert_eq!(cleaned[2].value, 100);
    }

    #[test]
    fn summarize_cleaned_data() {
        let cleaned = vec![
            CleanData { id: 1, value: 10 },
            CleanData { id: 2, value: 20 },
            CleanData { id: 3, value: 30 },
        ];

        let summary = summarize(&cleaned);

        assert_eq!(summary.total, 60);
        assert_eq!(summary.average, 20.0);
    }
}
