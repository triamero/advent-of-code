use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

pub fn compute(path_to_data: &str) -> i32 {
    let f = File::open(path_to_data).unwrap();
    let file = BufReader::new(&f);

    let mut result: (i32, i32) = (0, 0);

    for line in file.lines() {

        let row: String = line.unwrap();

        let line_result: (i32, i32) = process_line(row);

        result.0 = result.0 + line_result.0;
        result.1 = result.1 + line_result.1;
    }

    return result.0 * result.1;
}

fn process_line(line: String) -> (i32, i32) {
    let mut map : HashMap<char, i32> = HashMap::new();

    for c in line.chars() {

        *map.entry(c).or_insert(0) += 1;
    }

    let has_two: bool = map.values().filter(|x: &&i32| x == &&2i32).count() > 0;
    let has_three: bool = map.values().filter(|x| x == &&3i32).count() > 0;

    if has_two && has_three
    {
        return (1,1);
    }
    else if has_two {
        return (1, 0);
    }
    else if has_three {
        return (0,1);
    }


    return (0,0);
}