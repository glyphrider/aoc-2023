use core::cmp::max;
use regex::Regex;

fn main() {
    let lines = include_str!("../input.txt");
    let result1 = part1(lines,12,13,14);
    let result2 = part2(lines);
    println!("total1: {}", result1);
    println!("total2: {}", result2);
}

fn parse_draw(draw_string: &str) -> [u32; 3] {
    return draw_string.split(',').into_iter().fold(
        [0,0,0],|[r,g,b], cubes| {
            let els = cubes.trim().split(' ').collect::<Vec<&str>>();
            let count = els[0].parse().unwrap();
            return match els[1] {
                "red" => [count,g,b],
                "green" => [r,count,b],
                "blue" => [r,g,count],
                _ => [r,g,b],
            };
        });
}

fn parse_game(game_string: &str) -> [u32; 3] {
    return game_string.split(';').into_iter().fold(
        [0,0,0], | [r_max,g_max,b_max], draw_string | {
            let [r,g,b] = parse_draw(draw_string.trim());
            return [ max(r_max,r),max(g_max,g),max(b_max,b)];
        } 
    )
}

fn parse_line(line: &str) -> (u32,[u32;3]) {
    let re = Regex::new(r"^Game (\d+)$").unwrap();
    let parsed = line.split(':').map(|part| part.trim()).collect::<Vec<&str>>();
    let (_,[id_string]) = re.captures(parsed[0]).unwrap().extract();
    let game_id = id_string.parse::<u32>().unwrap();
    return (game_id, parse_game(parsed[1]));
}

fn part1(lines: &str, max_r: u32,max_g: u32,max_b: u32) -> u32 {
    return lines.lines().fold(0,|result, line| {
        let (game_id,[r,g,b]) = parse_line(line);
        if r <= max_r && g <= max_g && b <= max_b {
            return result + game_id;
        } else {
            return result;
        }
    });
}

fn part2(lines: &str) -> u32 {
    return lines.lines().fold(0,|power,line| {
        let (_game_id,[r,g,b]) = parse_line(line);
        return power + r*g*b;
    });
}

#[cfg(test)]
mod test_part1 {
    #[test]
    fn parse_draw_1_red_test() {
        assert_eq!(crate::parse_draw("1 red"),[1,0,0]);
    }
    #[test]
    fn parse_draw_1_red_1_green_test() {
        assert_eq!(crate::parse_draw("1 red, 1 green"),[1,1,0]);
    }
    #[test]
    fn parse_draw_1_green_test() {
        assert_eq!(crate::parse_draw("1 green"),[0,1,0]);
    }
    #[test]
    fn parse_draw_3_blue_1_green_2_red_test() {
        assert_eq!(crate::parse_draw("3 blue, 1 green, 2 red"),[2,1,3]);
    }

    #[test]
    fn parse_game1_contents_for_max_cube_counts_test() {
        assert_eq!(crate::parse_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),[4,2,6]);
    }

    #[test]
    fn parse_line_return_game_number_and_counts() {
        assert_eq!(crate::parse_line("Game 5: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),(5,[4,2,6]));
    }

    #[test]
    fn parse_input_sample_1_test() {
        assert_eq!(crate::part1(include_str!("../sample1.txt"),12,13,14),8);
    }

    #[test]
    fn parse_input_test() {
        assert_eq!(crate::part1(include_str!("../input.txt"),12,13,14),2617);
    }
}

#[cfg(test)]
mod test_part2 {

    #[test]
    fn parse_input_sample_1_test() {
        assert_eq!(crate::part2(include_str!("../sample1.txt")),2286);
    }
    #[test]
    fn parse_input_test() {
        assert_eq!(crate::part2(include_str!("../input.txt")),59795);
    }
}