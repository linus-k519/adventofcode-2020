use std::{collections::{HashMap, HashSet}, io::{stdin, Read}};

fn main() {
	let mut puzzle_input = String::new();
	stdin().read_to_string(&mut puzzle_input).unwrap();

	let (page_ordering_rules_str, pages_to_produce_str) = puzzle_input.split_once("\n\n").unwrap();

	// Stores for a page number which pages have to be printed before
	let mut page_ordering_rules = HashMap::<i64, HashSet<i64>>::new();
	for page_ordering_rule in page_ordering_rules_str.lines() {
		let (before, after) = page_ordering_rule.split_once('|').unwrap();
		let before = before.parse().unwrap();
		let after = after.parse().unwrap();
		page_ordering_rules.entry(after)
			.or_default() // Empty HashSet if not existing yet
			.insert(before); // insert rule into HashSet
	}

	let updates: Vec<Vec<i64>> = pages_to_produce_str.lines()
		.map(|line| {
			line.split(',')
				.map(|num| num.parse().unwrap())
				.collect()
		})
		.collect();

	let correct_updates_sum = updates.iter()
		.filter(|update| {
			update.iter().enumerate().all(|(i, page_nr)| {
				let mut printed_before_rules = page_ordering_rules.get(page_nr)
					.cloned()
					.unwrap_or_default(); // Empty HashSet of rules in case there are no rules for this page
				printed_before_rules.retain(|page_nr| update.contains(page_nr)); // "rules involving those missing page numbers are ignored"
				let actually_printed_before = update[..i].iter().copied().collect::<HashSet<_>>();
				actually_printed_before.is_subset(&printed_before_rules)
			})
		})
		.map(|update| {
			let middle_index = update.len() / 2;
			update[middle_index]
		})
		.sum::<i64>();

	dbg!(&correct_updates_sum);
}