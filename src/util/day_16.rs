use crate::util::Part;

pub fn solve(input:String, part:Part) -> String {
    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

fn part1(input:&str) -> String {
    let input = to_vec(input.trim());
    fft(input, 100)
}

fn part2(input:&str) -> String {
    let input = to_vec(input);
    fft(input, 100)
}

const BASE_PATTERN:[i32;4] = [0,1,0,-1];

fn get_base_pattern(index:u32, output_index:u32) -> i32 {
    let pattern_len = 4 * output_index;
    let pattern_index = ((index) % pattern_len) / output_index;
    BASE_PATTERN[pattern_index as usize]
}

fn fft(input:Vec<i32>,steps:usize) -> String {
    let mut next_input = input.clone();
    next_input.insert(0,0);

    for step in 1..(steps+1) {
        let mut tmp = vec![];
        tmp.insert(0, 0);
        for n in 1..next_input.len()+1 {
            let new_value : i32 = (next_input.iter().enumerate()
                .map(|(i, s)| s * get_base_pattern(i as u32, n as u32))
                .sum::<i32>() % 10).abs();

            tmp.push(new_value);
        }

        next_input = tmp;
        //next_input.remove(0);
        //next_input.iter_mut().enumerate().for_each( |(i, e)| *e = tmp[i] );
        //println!("{:?}", next_input);
    }

    next_input.remove(0);
    next_input.iter().map(|num| num.to_string()).collect::<String>()[..8].to_string()
}

fn test_period(input:Vec<i32>, times:usize) -> Vec<i32> {



    // Calc out position
    /*
    let mut next_input = input.clone();
    next_input.insert(0,0);
    let mut smallest_input_period = vec![];
    for i in 0..4* times {
        next_input.iter().for_each(|s| smallest_input_period.push(*s));
    }

    let mut tmp = vec![];
    for n in 1..smallest_input_period.len()+1 {
        let new_value : i32 = (smallest_input_period.iter().enumerate()
            .map(|(i, s)| s * get_base_pattern(i as u32, n as u32))
            .sum::<i32>());

        tmp.push(((new_value * (10_000 / 4)) % 10).abs() );
    }

    tmp

     */
    vec![]
}

fn to_vec(input:&str) -> Vec<i32> {
    let vector : Vec<i32> = input.chars().map( |ch| (ch as i32 - 0x30)).collect();
    vector
}

fn to_vec_times(input:&str,times:usize) -> Vec<i32> {
    let mut out:String = String::new();

    for i in 0..times {
        out.push_str(input.trim());
    }

    out.chars().map(|ch| (ch as i32 - 0x30)).collect()
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const input_data:&str ="59766299734185935790261115703620877190381824215209853207763194576128635631359682876612079355215350473577604721555728904226669021629637829323357312523389374096761677612847270499668370808171197765497511969240451494864028712045794776711862275853405465401181390418728996646794501739600928008413106803610665694684578514524327181348469613507611935604098625200707607292339397162640547668982092343405011530889030486280541249694798815457170337648425355693137656149891119757374882957464941514691345812606515925579852852837849497598111512841599959586200247265784368476772959711497363250758706490540128635133116613480058848821257395084976935351858829607105310340";

    #[test]
    fn test1() {
        let mut v = vec![];
        for i in 0..20 {
            v.push(get_base_pattern(i, 3));
        }

        println!("{:?}",v);
    }

    #[test]
    fn test2() {
        let input = vec![1,2,3,4,5,6,7,8];
        let res = fft(input, 4);

        println!("{:?}",res);
        assert_eq!("01029498", res);
    }

    #[test]
    fn test3() {
        let input = to_vec("80871224585914546619083218645595");
        let res = fft(input, 100);

        println!("{:?}",res);
        assert_eq!("24176176", res);
    }

    #[test]
    fn test4() {
        let input = to_vec("19617804207202209144916044189917");
        let res = fft(input, 100);

        println!("{:?}",res);
        assert_eq!("73745418", res);

    }


    #[test]
    fn test_part1() {
        let input = to_vec("59766299734185935790261115703620877190381824215209853207763194576128635631359682876612079355215350473577604721555728904226669021629637829323357312523389374096761677612847270499668370808171197765497511969240451494864028712045794776711862275853405465401181390418728996646794501739600928008413106803610665694684578514524327181348469613507611935604098625200707607292339397162640547668982092343405011530889030486280541249694798815457170337648425355693137656149891119757374882957464941514691345812606515925579852852837849497598111512841599959586200247265784368476772959711497363250758706490540128635133116613480058848821257395084976935351858829607105310340");
        let res = fft(input, 100);

        println!("{:?}",res);
        assert_eq!("18933364", res);
    }



    #[test]
    fn test_part2_test1() {
        let input_str = "03036732577212944063491565474664";
        let input = to_vec_times("03036732577212944063491565474664", 10);
        let res = fft(input.clone(), 100);

        println!("{:?}",res);

        let res2 = test_period(to_vec(input_str), 10);

        println!("{:?}", res2);
    }

    #[test]
    fn test_part2_test2() {
        let input = to_vec_times(input_data, 10);
        let res = fft(input, 10);

        println!("{:?}",res);
    }

    #[test]
    fn test_part2_test33() {
        let len = 651;
        let mut period = 4;

    }

    fn calc_times(index:i32, out_pos:i32, times:i32) {

        // 1,2,3,4,5,6,8,9,10

        // 0,1,0,-1,0  period = 4, occasions per input = 650 / 4 = 162, rest = 2
        // 0,0,1,1,0,0,-1,-1 period = 8, occasions = 10_000 / 8 = 1250
        // 0,0,0,1,1,1,0,0,0,-1,-1,-1 period = 12m, occasions  = 833, rest = 4

       // let period = (out_pos+1) * 4;
       // let occasions = times / period;
       // let rest = times % period;

    }
}
