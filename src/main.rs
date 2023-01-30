fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
struct Solution {}

#[cfg(test)]
fn is_palindrome(s: &str) -> bool {
    let length = s.len();
    let first_half = &s[..(length - length % 2) / 2];
    let second_half = &s[(length + length % 2) / 2..];
    if length == 1 {
        return true;
    }
    first_half
        .chars()
        .zip(second_half.chars().rev())
        .all(|(a, b)| a == b)
}

#[cfg(test)]
fn first_half(s: &str) -> &str {
    let length = s.len();
    &s[..(length - length % 2) / 2]
}

#[cfg(test)]
fn second_half(s: &str) -> &str {
    let length = s.len();
    &s[(length + length % 2) / 2..]
}

#[cfg(test)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let length = s.len();
        let mut answer = "";
        let mut max = 0;
        for start in 0..length {
            for end in start..(length + 1) {
                if end - start < max {
                    continue;
                }
                let slice = &s[start..end];

                if is_palindrome(slice) {
                    answer = slice;
                    max = slice.len();
                }
            }
        }
        String::from(answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case7first_half() {
        println!("looking for bb");
        assert_eq!(first_half("abcddcba"), "abcd");
        assert_eq!(first_half("abcdEdcba"), "abcd");
        assert_eq!(second_half("abcddcba"), "dcba");
        assert_eq!(second_half("abcdEdcba"), "dcba");
    }

    #[test]
    fn case2_is_palindrome() {
        println!("looking for bb");
        assert_eq!(is_palindrome("abcddcba"), true);
    }

    #[test]
    fn case1() {
        println!("looking for bcb");
        assert_eq!(
            Solution::longest_palindrome(String::from("abcabcbb")),
            "bcb"
        )
    }

    #[test]
    fn case_a() {
        println!("looking for a");
        assert_eq!(Solution::longest_palindrome(String::from("a")), "a")
    }

    #[test]
    fn case2() {
        println!("looking for bbbbb");
        assert_eq!(Solution::longest_palindrome(String::from("bbbbb")), "bbbbb")
    }

    #[test]
    fn case3() {
        println!("looking for ww");
        assert_eq!(Solution::longest_palindrome(String::from("pwwkew")), "ww")
    }

    #[test]
    fn case4() {
        println!("looking for aca");
        assert_eq!(Solution::longest_palindrome(String::from("aca")), "aca")
    }

    #[test]
    fn case5() {
        println!("looking for bbacaca");
        assert_eq!(Solution::longest_palindrome(String::from("acaca")), "acaca")
    }
}
