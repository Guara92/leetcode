#[test]
fn leetcode_test() {
    assert_eq!("leotcede", &Solution::reverse_vowels("leetcode".into()));
}

#[test]
fn hello_test() {
    assert_eq!("holle", &Solution::reverse_vowels("hello".into()));
}

struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels_map: Vec<(usize, char)> = vec![];
        let mut no_vowels = String::new();
        for (pos, ch) in s.chars().enumerate() {
            if is_vowel(ch) {
                vowels_map.push((pos, ch));
            } else {
                no_vowels.push(ch)
            }
        }

        let (pos, values): (Vec<usize>, Vec<char>) = vowels_map.into_iter().unzip();
        for (pos, ch) in pos.iter().zip(values.iter().rev()) {
            no_vowels.insert(*pos, *ch);
        }
        no_vowels
    }
}

fn is_vowel(ch: char) -> bool {
    matches!(
        ch,
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'
    )
}

fn main() {
    Solution::reverse_vowels("leetcode".into());
}
