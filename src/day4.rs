use std::collections::HashSet;

pub fn day4() {
  let max = 125730;
  let min = 579381;

  part1(max, min);
  part2(max, min);
}

fn part1(min: u32, max: u32) {
  println!("Day 4: Part 1");

  let mut possible_combinations = 0;
  for current_code in min..max + 1 {
    if is_valid_code(current_code) {
      possible_combinations += 1
    }
  }
  println!("Result part 1: {}", possible_combinations);
}

fn part2(min: u32, max: u32) {
  println!("Day 4: Part 2");

  let mut possible_combinations = 0;
  for current_code in min..max + 1 {
    if is_valid_code2(current_code) {
      possible_combinations += 1
    }
  }
  println!("Result part 2: {}", possible_combinations);
}

fn is_valid_code(code: u32) -> bool {
  let mut pair_equal_rule_fulfilled = false;
  for i in (0..5).rev() {
    let pair = code / 10_u32.pow(i) % 100;
    if pair % 11 == 0 {
      pair_equal_rule_fulfilled = true;
      break;
    }
  }

  let mut previous_digit = code / 10_u32.pow(5) % 10;
  let mut always_increasing_rule_fulfilled = true;
  for i in (0..5).rev() {
    let new_digit = code / 10_u32.pow(i) % 10;
    if new_digit < previous_digit {
      always_increasing_rule_fulfilled = false;
      break;
    }
    previous_digit = new_digit;
  }
  pair_equal_rule_fulfilled && always_increasing_rule_fulfilled
}

fn is_valid_code2(code: u32) -> bool {
  let mut pair_equal_rule_fulfilled = false;
  for i in (0..5).rev() {
    let pair = code / 10_u32.pow(i) % 100;
    let mut next_digit = 0;
    if (i > 0) {
      next_digit = code / 10_u32.pow(i - 1) % 10;
    }
    let prev_digit = code / 10_u32.pow(i + 2) % 10;

    if pair % 11 == 0 {
      let current_digit = pair / 11;

      if prev_digit != current_digit && next_digit != current_digit {
        pair_equal_rule_fulfilled = true;
        break;
      }
    }
  }

  let mut previous_digit = code / 10_u32.pow(5) % 10;
  let mut always_increasing_rule_fulfilled = true;
  for i in (0..5).rev() {
    let new_digit = code / 10_u32.pow(i) % 10;
    if new_digit < previous_digit {
      always_increasing_rule_fulfilled = false;
      break;
    }
    previous_digit = new_digit;
  }

  pair_equal_rule_fulfilled && always_increasing_rule_fulfilled
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    assert_eq!(is_valid_code(111111), true);
  }

  #[test]
  fn example_2() {
    assert_eq!(is_valid_code(223450), false);
  }

  #[test]
  fn example_3() {
    assert_eq!(is_valid_code(123789), false);
  }

  #[test]
  fn example_4() {
    assert_eq!(is_valid_code(111123), true);
  }

  #[test]
  fn example_5() {
    assert_eq!(is_valid_code(135679), false);
  }

  #[test]
  fn example_6() {
    assert_eq!(is_valid_code(111151), false);
  }
  #[test]
  fn example_7() {
    assert_eq!(is_valid_code2(123444), false);
  }
  #[test]
  fn example_8() {
    assert_eq!(is_valid_code2(111122), true);
  }
}
