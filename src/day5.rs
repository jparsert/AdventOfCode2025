use crate::puzzle::AoCPuzzle;

pub struct Day5 {
    pub filename : String
}


impl AoCPuzzle for Day5 {
    fn puzzle_name(&self) -> String {
        "Day 5".to_string()
    }

    fn first_puzzle(&self) -> i64 {
        let mut cnt: i64 = 0;
        let mut avail = false;

        let mut fresh_ranges: Vec<(i64, i64)> = Vec::new();

        for line in std::fs::read_to_string(&self.filename).unwrap().lines() {
            if !avail && line.is_empty() {
                avail = true;
                continue;
            }

            if !avail {
                let tmp = line.split("-").collect::<Vec<&str>>();
                let start = tmp[0].parse::<i64>().unwrap();
                let end = tmp[1].parse::<i64>().unwrap();
                fresh_ranges.push((start, end));
            } else {
                let el: i64 = line.parse::<i64>().unwrap();

                for &(start, end) in &fresh_ranges {
                    if start <= el && el <= end {
                        cnt += 1;
                        break;
                    }
                }
            }
        }
        cnt
    }

    fn second_puzzle(&self) -> Option<i64> {
        /*let mut cnt: i64 = 0;
        remove overlapping
        for line in std::fs::read_to_string(&self.filename).unwrap().lines() {
            if line.is_empty() {
                break;
            }

            let tmp = line.split("-").collect::<Vec<&str>>();
            let start = tmp[0].parse::<i64>().unwrap();
            let end = tmp[1].parse::<i64>().unwrap();
            
            cnt += end - start + 1
        }

        Some(cnt)
        
         */
        None
    }
}