use super::day;
use super::day_result::DayResult;
use std::cmp::{max, min};
use std::collections::HashSet;

pub struct Day3();

impl day::Day for Day3 {
    fn get_name(&self) -> String {
        return String::from("day3");
    }

    fn compute_first(&self, input: &Vec<String>) -> DayResult {
        let claims = input.iter().map(|x| Claim::new(x)).collect::<Vec<Claim>>();

        let mut map: HashSet<(i32, i32)> = HashSet::new();

        for c1 in &claims {
            for c2 in &claims {
                if c1.id == c2.id {
                    continue;
                }

                let points = c1.intersect(c2);

                for point in points {
                    map.insert(point);
                }
            }
        }


        return DayResult::from_i32(map.len() as i32);
    }

    fn compute_second(&self, input: &Vec<String>) -> DayResult {
        let claims = input.iter().map(|x| Claim::new(x)).collect::<Vec<Claim>>();

        let mut id = -1;

        for c1 in &claims {
            let mut intercect_with_some = false;

            for c2 in &claims {
                if c1.id == c2.id {
                    continue;
                }

                if is_rects_intercect(
                    &c1.top_left,
                    &c1.bottom_right,
                    &c2.top_left,
                    &c2.bottom_right,
                ) {
                    intercect_with_some = true;
                    break;
                }
            }

            if !intercect_with_some {
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
            coords[0].parse::<i32>().unwrap(),
            coords[1].parse::<i32>().unwrap(),
        );
        let bottom_right = Point::new(
            top_left.x + sizes[0].parse::<i32>().unwrap(),
            top_left.y + sizes[1].parse::<i32>().unwrap(),
        );

        return Claim {
            id: splits[0].trim_matches('#').parse::<i32>().unwrap(),

            top_left: top_left,
            bottom_right: bottom_right,
        };
    }

    fn intersect(&self, another_claim: &Claim) -> Vec<(i32, i32)> {
        if !is_rects_intercect(
            &self.top_left,
            &self.bottom_right,
            &another_claim.top_left,
            &another_claim.bottom_right,
        ) {
            return Vec::new();
        }

        // левая верхняя точка пересечения прямоугольников
        let top_left = Point::new(
            min(self.top_left.x, another_claim.top_left.x),
            min(self.top_left.y, another_claim.top_left.y),
        );
        // правая нижняя точка пересечения прямоугольников
        let bottom_right = Point::new(
            max(self.bottom_right.x, another_claim.bottom_right.x),
            max(self.bottom_right.y, another_claim.bottom_right.y),
        );

        let mut result: Vec<(i32, i32)> = Vec::new();

        for x in top_left.x..bottom_right.x + 1 {
            for y in top_left.y..bottom_right.y + 1 {
                result.push((x, y));
            }
        }

        return result;
    }
}

fn is_rects_intercect(
    top_left1: &Point,
    bottom_right1: &Point,
    top_left2: &Point,
    bottom_right2: &Point,
) -> bool {
    if is_point_intercect_rect(top_left1, bottom_right1, top_left2) {
        return true;
    }

    if is_point_intercect_rect(top_left1, bottom_right1, bottom_right2) {
        return true;
    }

    let top_right2 = Point::new(bottom_right2.x, top_left2.y);

    if is_point_intercect_rect(top_left1, bottom_right1, &top_right2) {
        return true;
    }

    let bottom_left2 = Point::new(top_left2.x, bottom_right2.y);

    if is_point_intercect_rect(top_left1, bottom_right1, &bottom_left2) {
        return true;
    }

    return false;
}

fn is_point_intercect_rect(top_left1: &Point, bottom_right1: &Point, point: &Point) -> bool {
    if top_left1.x <= point.x && point.x <= bottom_right1.x {
        if top_left1.y <= point.y && point.y <= bottom_right1.y {
            return true;
        }
    }

    return false;
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
