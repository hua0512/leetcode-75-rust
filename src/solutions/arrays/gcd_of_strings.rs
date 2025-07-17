struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        // A necessary and sufficient condition for a string GCD to exist is that
        // the two strings, when concatenated, are commutative.
        // If str1 = k*X and str2 = l*X, then str1+str2 = (k+l)*X and str2+str1 = (l+k)*X.
        // They must be equal. If they are not, no such base string X exists.
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return String::new();
        }

        // If a GCD string exists, its length must be the GCD of the lengths
        // of the input strings.
        let gcd_len = Self::gcd(str1.len(), str2.len());

        // The GCD string is the prefix of either string with the calculated GCD length.
        str1[..gcd_len].to_string()
    }

    /// Calculates the greatest common divisor of two numbers using the Euclidean algorithm.
    ///
    /// # Parameters
    /// - `a`: The first number.
    /// - `b`: The second number.
    ///
    /// # Returns
    /// The greatest common divisor of `a` and `b`.
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_of_strings() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABCDEF".to_string(), "ABC".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings(
                "TAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
                "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX".to_string()
            ),
            "TAUXX".to_string()
        );
    }
}
