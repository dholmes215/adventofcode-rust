//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for JunctionBox {
    fn from(line: &str) -> Self {
        let (x, y, z) = line
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        JunctionBox { x, y, z }
    }
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> f64 {
        ((((self.x - other.x) * (self.x - other.x))
            + ((self.y - other.y) * (self.y - other.y))
            + ((self.z - other.z) * (self.z - other.z))) as f64)
            .sqrt()
    }
}

pub fn day08(input: &str) -> SolutionResult {
    let boxes: Vec<JunctionBox> = input.lines().map_into().collect();
    let mut pairs = boxes
        .iter()
        .cartesian_product(boxes.iter())
        .filter(|(&l, &r)| l < r)
        .collect_vec();

    pairs.sort_by(|(l1, r1), (l2, r2)| l1.distance(r1).total_cmp(&l2.distance(r2)));

    type Circuit = Rc<RefCell<Vec<JunctionBox>>>;
    let mut circuits_by_members: BTreeMap<JunctionBox, Circuit> = BTreeMap::new();
    for &b in &boxes {
        circuits_by_members.insert(b, Rc::new(RefCell::from(vec![b])));
    }

    let is_example = boxes.len() < 1000;
    let boxes_to_connect = if is_example { 10 } else { 1000 };

    let mut connected_count = 0usize;
    let mut last_connection: Option<(JunctionBox, JunctionBox)> = None;
    let mut a: i64 = 0;
    while last_connection.is_none() {
        if connected_count >= pairs.len() {
            panic!("Ran out of pairs to connect");
        }
        let (l, r) = pairs[connected_count];
        let l_circuit_rc = circuits_by_members.get(l).cloned().unwrap();
        let r_circuit_rc = circuits_by_members.get(r).cloned().unwrap();
        if l_circuit_rc != r_circuit_rc {
            let mut r_circuit = r_circuit_rc.borrow_mut();

            {
                // Merge circuit r into l.
                let mut l_circuit = l_circuit_rc.borrow_mut();
                l_circuit.extend(r_circuit.iter().cloned());
                if l_circuit.len() == boxes.len() {
                    last_connection = Some((*l, *r))
                }
            }
            // Overwrite the Rcs for r
            for junction in r_circuit.iter() {
                circuits_by_members.insert(*junction, l_circuit_rc.clone());
            }
        }
        connected_count = connected_count + 1;

        if connected_count == boxes_to_connect {
            let mut circuits: Vec<Vec<JunctionBox>> = circuits_by_members
                .values()
                .map(|circuit| circuit.borrow().clone())
                .unique()
                .collect();
            circuits.sort_by(|l, r| r.len().cmp(&l.len()));
            a = circuits
                .iter()
                .take(3)
                .map(|circuit| circuit.len())
                .product::<usize>() as i64;
        }
    }

    let (b1, b2) = last_connection.unwrap();
    let b = b1.x * b2.x;

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}
