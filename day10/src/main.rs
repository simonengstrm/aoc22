fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    println!("{:?}",data[1][0]);

    let mut i = 0;
    let mut buffer = 0;
    let mut buffer_val = 0;
    let mut cycles = 0;
    let mut x_reg = 1;
    let mut signal_strength = 0;
    let mut drawing: Vec<&str> = Vec::new();

    loop {

        if buffer == 0 && buffer_val != 0 {
            // println!("Buffer 0 for {}", buffer_val);
            x_reg += buffer_val;
            buffer_val = 0;
            i += 1;
        }

        if cycles == 20 || (cycles > 40 && (cycles - 20) % 40 == 0) {
            signal_strength += x_reg * cycles;
        }

        // Draw
        let symbol = if (x_reg-1..x_reg+2).contains(&(cycles % 40)) {
            "#"
        } else {
            "."
        };
        drawing.push(symbol);

        if buffer > 0 {
            // println!("Reducing buffer {}", buffer);
            buffer -= 1;
            cycles += 1;
            continue;
        }

        if i == data.len()-1 {
            break;
        }

        match data[i][0] {
            "addx" => {
                buffer_val = data[i][1].parse::<i32>().unwrap();
                buffer = 1; // Started this cycle
                // println!("Addx found for {}", buffer_val);
            }
            _ => {
                i += 1;
            }
        }

        cycles += 1;
    }

    println!("{}, {}", signal_strength, cycles);
    for c in drawing {
        print!("{}", c);
    }
}
