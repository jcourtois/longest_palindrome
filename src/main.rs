fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let length = s.len();
        let mirror: String = s.chars().rev().collect();
        if length == 0 {
            return String::from("");
        }

        let mut answer = &s[..1];
        let mut max = 1;

        for start in 0..length {
            for end in start..=length {
                let slice_length = end - start;
                if slice_length < max || (slice_length - slice_length % 2) / 2 < 1 {
                    continue;
                }

                let first_half = &s[start..start + (slice_length - slice_length % 2) / 2];
                let second_half =
                    &mirror[length - end..length - start - (slice_length + slice_length % 2) / 2];

                if first_half == second_half {
                    answer = &s[start..end];
                    max = slice_length;
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
    fn case_1() {
        assert_eq!(
            Solution::longest_palindrome(String::from("iiiixfaaabaaaxx")),
            "aaabaaa"
        )
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::longest_palindrome(String::from("a")), "a")
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::longest_palindrome(String::from("iiiixfff")),
            "iiii"
        )
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::longest_palindrome(String::from("bbbbb")), "bbbbb")
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::longest_palindrome(String::from("pwwkew")), "ww")
    }

    #[test]
    fn case_6() {
        assert_eq!(Solution::longest_palindrome(String::from("aca")), "aca")
    }

    #[test]
    fn case_7() {
        assert_eq!(Solution::longest_palindrome(String::from("acaca")), "acaca")
    }
}
