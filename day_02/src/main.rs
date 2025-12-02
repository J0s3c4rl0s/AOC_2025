const input_str : &str = 
    // "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    "989133-1014784,6948-9419,13116184-13153273,4444385428-4444484883,26218834-26376188,224020-287235,2893-3363,86253-115248,52-70,95740856-95777521,119-147,67634135-67733879,2481098640-2481196758,615473-638856,39577-47612,9444-12729,93-105,929862406-930001931,278-360,452131-487628,350918-426256,554-694,68482544-68516256,419357748-419520625,871-1072,27700-38891,26-45,908922-976419,647064-746366,9875192107-9875208883,3320910-3352143,1-19,373-500,4232151-4423599,1852-2355,850857-889946,4943-6078,74-92,4050-4876";

fn main() {
    let mut sum = 0;
    let id_list = input_str.split(',');
    for id_pair in id_list {
        let mut id_pairs = id_pair.split('-');
        let from : usize = id_pairs.next().expect("First doesnt exist").parse().expect("First ID not a number");
        let to = id_pairs.next().expect("Second doesnt exist").parse().expect("Second ID not a number");
        
        for id in from..=to {
            let id_str = id.to_string();
            if check_id2(&id_str) {
                // println!("Invalid ID: {}", id);
                sum += id;
            }
        }
    }
    println!("{}", sum);
}

fn check_id(id_str: &str) -> bool {
    if id_str.len() > 1 && id_str.len() % 2 == 0 {
        // let chars = id_str.chars();
        let mid = id_str.len() / 2;
        id_str[0..mid] == id_str[mid..]
        // true
    }
    else {
        false
    }
}

fn check_id2(id_str: &str) -> bool {
    if id_str.len() > 1 {
        (2..=id_str.len()).any(|size| check_id2_helper(id_str, size))
    }
    else {
        false
    }
}

fn check_id2_helper(id_str: &str, divisor : usize) -> bool {
    if id_str.len() % divisor != 0 {
        return false;
    }

    let chunk_size = id_str.len() / divisor;

    let sub_str = &id_str[0..chunk_size];

    // println!("Checking string {id_str} for sub_str {sub_str}");

    (1..divisor).all(|i| sub_str == &id_str[(i * chunk_size)..(i * chunk_size + chunk_size)])

    // for i in  {
    //     let start = i * chunk_size;
    //     let end = start + chunk_size;
        
    // }
}