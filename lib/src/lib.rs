/// High rank is bad
pub fn rank(term: &str, s: &str) -> Option<usize> {
    let mut score = 0;
    let mut s_iter = s.chars();
    for (i, ch) in term.chars().enumerate() {
        loop {
            let Some(next) = s_iter.next() else {
                return None;
            };
            if next == ch {
                break;
            }
            if i == 0 {
                // Letters before first match are worth 1
                score += 1;
            } else {
                // Letters inside of match are worth 10
                score += 10
            }
        }
    }

    // Letters after match are also worth 1
    Some(score + s_iter.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rank("ab", "abc"), Some(1));
        assert_eq!(rank("abc", "ab"), None);
        assert_eq!(rank("abc", "ab and c"), Some(50));
        assert_eq!(rank("abc", "beforeabc"), Some(6));
    }
}
