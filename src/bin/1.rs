fn height(s: &String) -> isize {
    s.as_str().chars().into_iter().filter(|c| *c == '(').count() as isize
        - s.as_str().chars().into_iter().filter(|c| *c == ')').count() as isize
}

fn height2(s: &String) -> Option<usize> {
    let mut pos: isize = 0;
    for (i, c) in s.chars().into_iter().enumerate() {
        match c {
            '(' => pos += 1,
            ')' => pos -= 1,
            _ => continue,
        }
        if pos == -1 {
            return Some(i + 1);
        }
    }
    None
}

fn read_file(file: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file)?.parse::<String>()?)
}

fn read_inputs(num_ins: u8) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok((1..=num_ins)
        .into_iter()
        .map(|n| {
            read_file(&format!("inputs/{:?}_in.txt", n))
                .expect(&format!("failed to read in input file = {:?}", n))
        })
        .collect::<Vec<String>>())
}

fn read_outputs(num_ins: u8) -> Result<Vec<isize>, Box<dyn std::error::Error>> {
    Ok((1..=num_ins)
        .into_iter()
        .map(|n| {
            read_file(&format!("inputs/{:?}_out.txt", n))
                .expect(&format!("failed to read in input file = {:?}", n))
        })
        .map(|s| {
            s.parse::<isize>().expect(&format!(
                "failed to convert the given output value into an integer = {s}"
            ))
        })
        .collect::<Vec<isize>>())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn inputs() {
        let inputs = read_inputs(4).expect("failed to read in all the input files");
        let outputs = read_outputs(4).expect("failed to read in all the output files");
        for (inp, out) in inputs.into_iter().zip(outputs.into_iter()).into_iter() {
            assert_eq!(height(&inp), out);
        }
    }
}

fn main() {
    let file = read_file("inputs/in.txt").expect("failed to read in the file inputs/in.txt");
    println!("santa's final height = {}", height(&file));
    println!(
        "santa enters basement at step = {:#?}",
        height2(&"()())".to_string())
    );
    println!(
        "santa enters basement at step = {:#?}",
        height2(&")".to_string())
    );
    println!("santa enters basement at step = {:#?}", height2(&file));
}
