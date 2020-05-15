pub struct DayResult {
    pub ival: Option<i32>,
    pub sval: Option<String>,
}

impl DayResult {
    pub fn from_i32(val: i32) -> DayResult {
        return DayResult {
            ival: Some(val),
            sval: None
        };
    }

    pub fn from_string(val: String) -> DayResult {
        return DayResult {
            ival: None,
            sval: Some(val)
        };
    }
}