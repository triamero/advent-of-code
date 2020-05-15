use super::day;
use super::day_result::DayResult;

pub struct Day3();

impl day::Day for Day3 {
    fn get_name(&self) -> String {
        return String::from("day3");
    }

    fn compute_first(&self, input: &Vec<String>) -> DayResult {
        let claims = input.iter().map(|x| Claim::new(x)).collect::<Vec<Claim>>();

        let mut fabric = create_fabric();

        for c1 in &claims {
            for i in c1.top_left.x..c1.bottom_right.x + 1 {
                for j in c1.top_left.y..c1.bottom_right.y + 1 {
                    fabric[i as usize][j as usize].claim_ids.push(c1.id);
                }
            }
        }

        let mut more_than_one_claim_count = 0;

        for i in 0..1001 {
            for j in 0..1001 {
                let count = fabric[i][j].claim_ids.len();

                if count > 1 {
                    more_than_one_claim_count += 1;
                }
            }
        }
        return DayResult::from_i32(more_than_one_claim_count);
    }

    fn compute_second(&self, input: &Vec<String>) -> DayResult {
        let claims = input.iter().map(|x| Claim::new(x)).collect::<Vec<Claim>>();

        let mut fabric = create_fabric();

        for c1 in &claims {
            for i in c1.top_left.x..c1.bottom_right.x + 1 {
                for j in c1.top_left.y..c1.bottom_right.y + 1 {
                    fabric[i as usize][j as usize].claim_ids.push(c1.id);
                }
            }
        }
        let mut id = -1;

        for c1 in &claims {
            let mut same_id = true;

            for i in c1.top_left.x..c1.bottom_right.x + 1 {
                for j in c1.top_left.y..c1.bottom_right.y + 1 {
                    
                    if fabric[i as usize][j as usize].claim_ids.len() != 1 {
                        same_id = false;
                        break;
                    }
                }

                if !same_id {
                    break;
                }
            }

            if same_id {
                id = c1.id;
                break;
            }
        }

        return DayResult::from_i32(id);
    }
}

struct Claim {
    /// Идентификатор
    id: i32,

    /// Левая верхняя точка
    top_left: Point,

    /// Правая нижняя точка
    bottom_right: Point,
}

impl Claim {
    fn new(line: &str) -> Claim {
        // #12 @ 512,893: 16x22
        // [0]=#12 [1]=@ [2]=512,893: [3]=16x22
        let splits = line.trim().split(" ").collect::<Vec<&str>>();
        let coords: Vec<&str> = splits[2].trim_matches(':').split(',').collect();

        let sizes: Vec<&str> = splits[3].split('x').collect();

        let top_left = Point::new(
            coords[0].parse::<i32>().unwrap() + 1,
            coords[1].parse::<i32>().unwrap() + 1,
        );
        let bottom_right = Point::new(
            top_left.x + sizes[0].parse::<i32>().unwrap() - 1,
            top_left.y + sizes[1].parse::<i32>().unwrap() - 1,
        );

        return Claim {
            id: splits[0].trim_matches('#').parse::<i32>().unwrap(),

            top_left: top_left,
            bottom_right: bottom_right,
        };
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

struct Inch {
    claim_ids: Vec<i32>,
}

impl Inch {
    fn new() -> Inch {
        return Inch {
            claim_ids: Vec::new(),
        };
    }
}


fn create_fabric() -> Vec<Vec<Inch>> {
    let mut fabric: Vec<Vec<Inch>> = Vec::new();

    for _i in 0..1001 {
        let mut vec: Vec<Inch> = Vec::new();
        for _j in 0..1001 {

            vec.push(Inch::new());
        }
        fabric.push(vec);
    }

    fabric
}