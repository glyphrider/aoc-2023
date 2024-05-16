struct ColorCount {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let lines: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut result1 = 0;
    let mut result2 = 0;
    let mut count: ColorCount = ColorCount {
        red: 0,
        blue: 0,
        green: 0,
    };
    for line in lines {
        println!("line = {}", line);
        let game_number = extract_game(&line);
        if get_max_colors(&line, &mut count) {
            result1 = result1 + game_number;
        }
        result2 = result2 + (count.red * count.green * count.blue);
    }
    println!("total1: {}", result1);
    println!("total2: {}", result2);
}

fn split_at_colon(line: &str) -> Vec<&str> {
    return line.split(":").map(str::trim).collect();
}

fn extract_game(line: &str) -> u32 {
    let v: Vec<&str> = split_at_colon(line);
    // println!("v = {},{}",v[0],v[1]);
    let game_number = u32::from_str_radix(&v[0][5..], 10).unwrap();
    return game_number;
}

fn get_max_colors(line: &str, count: &mut ColorCount) -> bool {
    let mut result = true;
    let v = split_at_colon(line);
    let draws: Vec<&str> = v[1].split(";").collect();
    count.red = 0;
    count.blue = 0;
    count.green = 0;
    for draw in draws {
       println!("draw = {}", draw);
        let cubes: Vec<&str> = draw.split(",").map(str::trim).collect();
        for cube in cubes {
            println!("  cube = {}", cube);
            let cube_split: Vec<&str> = cube.split(" ").map(str::trim).collect();
            let cube_count = u32::from_str_radix(cube_split[0], 10).unwrap();
            match cube_split[1] {
                "red" => count.red = count.red.max(cube_count),
                "blue" => count.blue = count.blue.max(cube_count),
                "green" => count.green = count.green.max(cube_count),
                &_ => println!("  *** bad color ***"),
            }
        }
        if count.red > 12 {
            result = false;
        }
        if count.green > 13 {
            result = false;
        }
        if count.blue > 14 {
            result = false;
        }
    }
    return result;
}
