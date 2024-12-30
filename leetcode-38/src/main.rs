struct Solution;

fn main() {}

fn rle<T>(s: T) -> String
where
    T: AsRef<str>
{
    let mut ret = String::new();

    let mut curr_char = ' ';
    let mut count = 0;
    
    for c in s.as_ref().chars() {
        if c == curr_char {
            count += 1;
        } else {
            if count > 0 {
                ret.push_str(&count.to_string());
                ret.push(curr_char);
            }

            count = 1;
            curr_char = c;
        }
    }

    if count > 0 {
        ret.push_str(&count.to_string());
        ret.push(curr_char);
    }

    ret
}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            String::from("1")
        } else {
            rle(Self::count_and_say(n - 1))
        }
    }
}
