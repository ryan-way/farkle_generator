use std::collections::HashSet;

fn generate(set: &mut Vec<u64>, output: &mut Vec<Vec<u64>>, start: usize) {
    if start == set.len() {
        output.push(set.clone());
        return;
    }

    for i in 0..6 {
        set[start] = (i+1) as u64;
        generate(set, output, start+1);
    }
}

fn generate_data_set(num: u64) -> Vec<Vec<u64>> {
    let mut set = [0].repeat(num as usize);
    let mut output = vec![];

    generate(&mut set, &mut output, 0);

    output
}

fn score_set(data: &Vec<u64>, scores: &Vec<u64>) -> u64 {
    let mut score: u64 = 0;
    let mut num_used = 0;

    let mut set: HashSet<u64> = HashSet::new();
    for d in data.iter() {
        set.insert(*d);
    }

    if data.len() == set.len() && data.len() == 6 {
        return 1500 + scores[5];
    }

    for num in 1..7 {
        if data.iter().filter(|&d| *d == num as u64).count() == 3 {
            num_used += 3;
            score += num * 100;
        }
    }


    for [num, val] in [[1, 100], [5, 50]].iter() {
        let count = data.iter().filter(|&d| *d == *num).count() as u64;
        let count = count % 3;
        num_used += count;
        score += count * val;
    }

    if num_used > data.len() as u64 {
        println!("ERROR: {} out of {}", num_used, data.len());
        println!("data: {}", data.iter().map(|&d| d.to_string()).collect::<Vec<String>>().join(" "));
    }

    score += match num_used {
        0 => 0,
        6 => scores[5],
        _ => scores[5-num_used as usize]
    };

    score
}

fn main() {

    let mut previous: Vec<u64> = [0].repeat(6);
    let mut scores: Vec<u64> = [0].repeat(6);
    loop {
        for num in 1..7 {
            let data_set = generate_data_set(num);

            scores[(num-1) as usize] = data_set.iter().map(|data| score_set(data, &previous)).sum();

            println!("Total score for {}: {}", num, (scores[(num-1) as usize] as f64)/(data_set.len() as f64));

        }

        if previous == scores {
            break;
        }

        previous = scores.clone();
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_of_one_should_generate_output_of_len_one() {
        let mut set = vec![0];
        let mut output = vec![];

        generate(&mut set, &mut output, 0);

        assert_eq!(output.len(), 1);
    }

    #[test]
    fn generate_of_two_should_generate_output_of_len_four() {
        let mut set = vec![0, 0];
        let mut output = vec![];

        generate(&mut set, &mut output, 0);

        assert_eq!(output.len(), 4);
    }

    #[test]
    fn generate_of_three_should_generate_output_of_len_27() {
        let mut set = vec![0, 0, 0];
        let mut output = vec![];

        generate(&mut set, &mut output, 0);

        assert_eq!(output.len(), 27);
    }


    #[test]
    fn generate_of_six_should_generate_output_of_len_alot() {
        let mut set = vec![0, 0, 0, 0, 0, 0];
        let mut output = vec![];

        generate(&mut set, &mut output, 0);

        assert_eq!(output.len(), usize::pow(6, 6));
    }
}
