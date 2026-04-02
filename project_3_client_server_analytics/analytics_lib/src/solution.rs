use crate::dataset::Dataset;
use crate::dataset::Value;
use crate::query::{Aggregation, Condition};
use std::collections::HashMap;

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    // Student 1 will implement this
    todo!()
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    // Student 1 will implement this
    todo!()
}

pub fn aggregate_dataset(
    dataset: HashMap<Value, Dataset>,
    aggregation: &Aggregation,
) -> HashMap<Value, Value> {
    let mut result = HashMap::new();
    
    match aggregation {
        Aggregation::Count(_col_name) => {
            for (group_key, group_dataset) in dataset {
                result.insert(group_key, Value::Integer(group_dataset.len() as i32));
            }
        }
        Aggregation::Sum(col_name) | Aggregation::Average(col_name) => {
            let col_index = dataset.values()
                .next()
                .map(|group_dataset| group_dataset.column_index(col_name));
            
            if let Some(idx) = col_index {
                for (group_key, group_dataset) in dataset {
                    let mut sum = 0i32;
                    let mut count = 0usize;
                    
                    for i in 0..group_dataset.len() {
                        let row = group_dataset.iter().nth(i).unwrap();
                        if let Value::Integer(num) = row.get_value(idx) {
                            sum += num;
                            count += 1;
                        }
                    }
                    
                    let value = match aggregation {
                        Aggregation::Sum(_) => Value::Integer(sum),
                        Aggregation::Average(_) => {
                            if count > 0 {
                                Value::Integer(sum / count as i32)
                            } else {
                                Value::Integer(0)
                            }
                        }
                        _ => unreachable!(),
                    };
                    
                    result.insert(group_key, value);
                }
            }
        }
    }
    
    result
}