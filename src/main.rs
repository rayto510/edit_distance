#[allow(dead_code)]
fn edit_distance(word1: String, word2: String) -> i32 {
    // word1 = starting word
    // word2 = target word
    //
    // return minimum number of operations required to convert word1 to word2

    fn go(i: usize, j: usize, dp: &mut Vec<Vec<i32>>, word1: &Vec<char>, word2: &Vec<char>) -> i32 {
        // if word1 is empty, we need to insert all characters from word2
        if i == 0 {
            return j as i32;
        }

        // if word2 is empty, we need to remove all characters from word1
        if j == 0 {
            return i as i32;
        }

        // if we have already calculated the answer, return it
        if dp[i][j] != -1 {
            return dp[i][j];
        }

        // if the characters are the same, we don't need to do anything
        if word1[i - 1] == word2[j - 1] {
            go(i - 1, j - 1, dp, word1, word2)

        // if the characters are different, we need to choose the operation that gives us the minimum number of operations
        } else {
            let add = 1 + go(i, j - 1, dp, word1, word2);
            let remove = 1 + go(i - 1, j, dp, word1, word2);
            let replace = 1 + go(i - 1, j - 1, dp, word1, word2);
            dp[i][j] = add.min(remove).min(replace);
            dp[i][j]
        }
    }

    let word1: Vec<char> = word1.chars().collect();
    let word2: Vec<char> = word2.chars().collect();
    let n = word1.len();
    let m = word2.len();
    let mut dp = vec![vec![-1; m + 1]; n + 1];
    go(n, m, &mut dp, &word1, &word2)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(
            edit_distance("intention".to_string(), "execution".to_string()),
            5
        );
        // really long strings
        assert_eq!(
            edit_distance(
                "pneumonoultramicroscopicsilicovolcanoconiosis".to_string(),
                "ultramicroscopically".to_string()
            ),
            27
        );
    }
}
