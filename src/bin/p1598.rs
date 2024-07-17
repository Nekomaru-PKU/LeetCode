mod solution {
    #[allow(dead_code)]
    pub fn main(logs: Vec<String>) -> i32 {
        let mut depth = 0;
        for log in &logs {
            match log.as_str() {
                "../" => depth = (depth - 1).max(0),
                "./" => (),
                _ => depth += 1,
            }
        }
        depth
    }
}

fn main() {}