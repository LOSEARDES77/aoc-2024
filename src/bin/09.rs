advent_of_code::solution!(9);

#[derive(Debug)]
struct Disk {
    compressed: String,
    uncompressed: Option<Vec<String>>,
    defragmented: Option<Vec<String>>,
}

impl Disk {
    pub fn new(compress_data: &str) -> Self {
        Self {
            compressed: compress_data.to_string(),
            uncompressed: None,
            defragmented: None,
        }
    }

    pub fn decompress(&mut self) -> Option<Result<(), ()>> {
        let mut uncompressed_disk = Vec::new();
        let mut free = false;
        let mut id = 0;
        for c in self.compressed.trim().chars() {
            let n = c.to_digit(10)?;
            if free {
                for _ in 0..n {
                    uncompressed_disk.push(".".to_string());
                }
            } else {
                for _ in 0..n {
                    uncompressed_disk.push(id.to_string());
                }
                id += 1;
            }
            free = !free
        }

        self.uncompressed = Some(uncompressed_disk);

        Some(Ok(()))
    }

    pub fn defrag(&mut self) -> Option<Result<(), ()>> {
        let mut defrag = self.uncompressed.clone()?;
        for i in defrag.len()..0 {
            if defrag[i] != "." {
                for j in 0..defrag.len() {
                    if defrag[j] == "." {
                        defrag[j] = defrag[i].clone();
                        defrag[i] = ".".to_string();
                        break;
                    }
                }
            }
        }
        self.defragmented = Some(defrag);
        Some(Ok(()))
    }

    pub fn get_checksum(&self) -> Option<u32> {
        let mut sum = 0;
        for (i, num) in self.defragmented.clone()?.iter().enumerate() {
            if *num == "." {
                break;
            }
            sum += i as u32 * num.parse::<u32>().unwrap()
        }
        Some(sum)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut disk = Disk::new(input);
    disk.decompress()?.unwrap();
    disk.defrag()?.unwrap();
    println!("{:?}", disk);
    //println!("{:?}", disk);
    disk.get_checksum()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_09_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
