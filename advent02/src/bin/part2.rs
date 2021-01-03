#[derive(PartialEq, Debug)]
struct PasswordPolicy {
    byte: u8,
    positions: [usize; 2],
}

fn parse_line(s: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    peg::parser! {
      grammar parser() for str {
        rule number() -> usize
          = n:$(['0'..='9']+) { n.parse().unwrap() }

        /// Positions are 1-based indices in the input
        rule position() -> usize
          = n:number() { n - 1 }

        rule positions() -> [usize; 2]
          // now using `position()` rather than `number()`
          = first:position() "-" second:position() { [first, second] }

        rule byte() -> u8
          = letter:$(['a'..='z']) { letter.as_bytes()[0] }

        rule password() -> &'input str
          = letters:$([_]*) { letters }

        pub(crate) rule line() -> (PasswordPolicy, &'input str)
            = positions:positions() " " byte:byte() ": " password:password() {
              (PasswordPolicy { positions, byte }, password)
          }
      }
    }

    Ok(parser::line(s)?)
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.positions
            .iter()
            .copied()
            .filter(|&index| password.as_bytes()[index] == self.byte)
            .count()
            == 1
    }
}

#[cfg(test)]
mod tests {
    use super::PasswordPolicy;

    #[test]
    fn test_is_valid() {
        let pp = PasswordPolicy {
            range: 1..=3,
            byte: b'a',
        };
        assert_eq!(pp.is_valid("zeus"), false, "no 'a's");
        assert_eq!(pp.is_valid("hades"), true, "single 'a'");
        assert_eq!(pp.is_valid("banana"), true, "three 'a's");
        assert_eq!(pp.is_valid("aaaah"), false, "too many 'a's");
    }
}

fn main() -> anyhow::Result<()> {
    let count = include_str!("../input.txt")
        .lines()
        .map(parse_line)
        .map(Result::unwrap)
        .filter(|(policy, password)| policy.is_valid(password))
        .count();
    println!("{} passwords are valid", count);

    Ok(())
}
