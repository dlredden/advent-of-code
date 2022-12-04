use array_tool::vec::Intersect;
use itertools::Itertools;

pub fn find_badges(group: Vec<&str>) -> Vec<char> {
    group[0]
        .chars()
        .collect_vec()
        .intersect(group[1].chars().collect_vec())
        .intersect(group[2].chars().collect_vec())
        .into_iter()
        .unique()
        .collect()
}

pub fn find_common_items(half1: &str, half2: &str) -> Vec<char> {
    half1
        .chars()
        .collect_vec()
        .intersect(half2.chars().collect_vec())
        .into_iter()
        .unique()
        .collect()
}

pub static PRIORITIES: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
