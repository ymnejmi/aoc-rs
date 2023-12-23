use std::{collections::HashMap, fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let digits = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];
    let _str_digits: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let mut input = File::open("input.txt")?;
    let mut str = String::new();
    input.read_to_string(&mut str)?;
    let res = str.lines().map(|line| {
        // println!("{}", line);
        let mut first = "";
        let mut last = "";

        let mut findex = line.len();
        let mut rindex = 0;
        for digit in digits {
            let first_occurrence = line.find(digit).unwrap_or(usize::MAX);
            if first_occurrence <= findex {
                first = digit;
                findex = first_occurrence
                // println!("f{}: {}", digit, line.find(digit).unwrap_or(findex));
            }
            let last_occurrence = line.rfind(digit).unwrap_or(usize::MAX);

            // if line == "6lpxrrhvdslxgpjblcmgsgbdpdkfmzkr" {
            //     println!("{line}: {:>5}[{}]", digit, line.rfind(digit).unwrap_or(0));
            // }
            if last_occurrence != usize::MAX && last_occurrence >= rindex {
                last = digit;
                rindex = last_occurrence;
            }
        }
        // println!("{first:<6} {line:^40} {last:>6}");
        // println!(
        //     "{:<24} {first:<6} {line:^64} {last:>6} {:32} :{}{}",
        //     line.split_once(first).unzip().0.unwrap_or(""),
        //     line.rsplit_once(last).unzip().1.unwrap_or(""),
        //     _str_digits.get(first).unwrap_or(&first),
        //     _str_digits.get(last).unwrap_or(&last)
        // );
        format!(
            "{}{}",
            _str_digits.get(first).unwrap_or(&first),
            _str_digits.get(last).unwrap_or(&last)
        )
        .parse()
        .unwrap_or(0)
    });
    // for (line, v) in zip(str.lines(), res) {
    //     println!("{}: {}", line, v);
    // }
    println!("{}", res.sum::<i32>());
    Ok(())
}
