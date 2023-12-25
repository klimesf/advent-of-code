use std::collections::VecDeque;

pub(crate) fn day14() {
    println!("{}", part_a(409551));
    println!("{}", part_b("409551".to_string()));
}

fn part_a(input: usize) -> String {
    let mut recipes = vec![3, 7];
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    while recipes.len() < input + 10 {
        let new_recipe = recipes[elf_1] + recipes[elf_2];
        for d in digits(new_recipe) {
            recipes.push(d);
        }

        elf_1 = (elf_1 + recipes[elf_1] + 1) % recipes.len();
        elf_2 = (elf_2 + recipes[elf_2] + 1) % recipes.len();
    }


    let mut res = String::new();
    for i in 0..10 {
        res.push(char::from_digit(recipes[input + i] as u32, 10).unwrap());
    }
    return res;
}

fn part_b(input: String) -> usize {
    let mut recipes = vec![3, 7];
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    while contains_string(&recipes, &input) == 0 {
        let new_recipe = recipes[elf_1] + recipes[elf_2];
        for d in digits(new_recipe) {
            recipes.push(d);
        }

        elf_1 = (elf_1 + recipes[elf_1] + 1) % recipes.len();
        elf_2 = (elf_2 + recipes[elf_2] + 1) % recipes.len();
    }

    return contains_string(&recipes, &input);
}

fn digits(mut new_recipe: usize) -> Vec<usize> {
    let mut digits = vec!();
    loop {
        if new_recipe < 10 { break; }
        digits.push(new_recipe % 10);
        new_recipe = new_recipe / 10;
    }
    digits.push(new_recipe);
    digits.reverse();
    return digits
}

fn contains_string(recipes: &Vec<usize>, input: &String) -> usize {
    if recipes.len() < input.len() + 2 { return 0; }
    let mut deque = VecDeque::new();
    let mut i = recipes.len();
    let mut found = false;
    while i > recipes.len() - input.len() - 2 {
        i -= 1;
        deque.push_front(recipes[i]);
        if deque.len() > input.len() {
            deque.pop_back();
        }

        let mut s = String::new();
        deque.iter().for_each(|c| s.push(char::from_digit(*c as u32, 10).unwrap()));

        if s == *input {
            found = true;
            break;
        }
    }

    return if found { i } else { 0 };
}

#[cfg(test)]
mod day14_tests {
    use crate::y2018::day14::{contains_string, digits, part_a, part_b};

    #[test]
    fn part_a_works() {
        assert_eq!("5158916779", part_a(9));
        assert_eq!("0124515891", part_a(5));
        assert_eq!("9251071085", part_a(18));
        assert_eq!("5941429882", part_a(2018));
    }

    #[test]
    fn part_b_works() {
        assert_eq!(9, part_b("51589".to_string()));
        assert_eq!(5, part_b("01245".to_string()));
        assert_eq!(18, part_b("92510".to_string()));
        assert_eq!(2018, part_b("59414".to_string()));
    }

    #[test]
    fn digits_works() {
        assert_eq!(vec![0], digits(0));
        assert_eq!(vec![1], digits(1));
        assert_eq!(vec![9], digits(9));
        assert_eq!(vec![1, 0], digits(10));
        assert_eq!(vec![1, 1], digits(11));
        assert_eq!(vec![1, 8], digits(18));
    }

    #[test]
    fn contains_string_works() {
        assert_eq!(3, contains_string(&vec![0, 0, 0, 0, 0, 1, 0], &"001".to_string()));
        assert_eq!(3, contains_string(&vec![0, 0, 0, 0, 0, 1], &"001".to_string()));
        assert_eq!(0, contains_string(&vec![0, 0, 0, 0, 0, 0, 0], &"001".to_string()));
    }
}
