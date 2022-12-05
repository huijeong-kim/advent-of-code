use std::fs::File;
use std::io::BufRead;

#[derive(Debug)]
struct Command {
    num_crates: i32,
    from: i32,
    to: i32,
}

fn main() {
    let stacks = get_initial_stacks();
    let commands = get_day5_command_input();

    {
        let mut stacks = stacks.clone();
        for command in &commands {
            for _ in 0..command.num_crates {
                let value = stacks[command.from as usize - 1].pop().unwrap();
                stacks[command.to as usize - 1].push(value);
            }
        }
        let result: String = stacks
            .into_iter()
            .map(|mut vec| vec.pop().unwrap())
            .collect();
        println!("{}", result);
    }

    {
        let mut stacks = stacks.clone();
        for command in &commands {
            let mut crates_to_move = Vec::new();
            for count in 0..command.num_crates {
                let value = stacks[command.from as usize - 1].pop().unwrap();
                crates_to_move.push(value);
            }

            for _ in 0..command.num_crates {
                let value = crates_to_move.pop().unwrap();
                stacks[command.to as usize - 1].push(value);
            }
        }
        let result: String = stacks
            .into_iter()
            .map(|mut vec| vec.pop().unwrap())
            .collect();
        println!("{}", result);
    }
}

fn get_initial_stacks() -> Vec<Vec<char>> {
    vec![
        vec!['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'], // 1
        vec!['N', 'V', 'G', 'P', 'H', 'W', 'B'],      // 2
        vec!['F', 'W', 'B', 'J', 'G'],                // 3
        vec!['G', 'J', 'N', 'F', 'L', 'W', 'C', 'S'], // 4
        vec!['W', 'J', 'L', 'T', 'P', 'M', 'S', 'H'], // 5
        vec!['B', 'C', 'W', 'G', 'F', 'S'],           // 6
        vec!['H', 'T', 'P', 'M', 'Q', 'B', 'W'],      // 7
        vec!['F', 'S', 'W', 'T'],                     // 8
        vec!['N', 'C', 'R'],                          // 9
    ]
}

fn get_day5_command_input() -> Vec<Command> {
    let file = File::open("inputs/day5.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.starts_with('#')).collect();

    let mut commands = Vec::new();
    for line in lines {
        let values: Vec<&str> = line.split(" ").collect();

        commands.push(Command {
            num_crates: values[1].parse::<i32>().unwrap(),
            from: values[3].parse::<i32>().unwrap(),
            to: values[5].parse::<i32>().unwrap(),
        })
    }
    commands
}
