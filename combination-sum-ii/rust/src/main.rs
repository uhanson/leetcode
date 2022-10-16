use std::collections::BTreeMap;

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn recurse(cs: &mut BTreeMap<i32, i32>, target: i32, comb: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if target == 0 {
            ret.push(comb.clone());
        } else { 
            let keys = cs.keys().cloned().collect::<Vec<_>>();
            for k in keys.iter() {
                if *cs.get(&k).unwrap() > 0 && target - k >= 0 {
                    let mut ccs = cs.clone();
                    cs.entry(*k).and_modify(|n| *n = 0);
                    ccs.entry(*k).and_modify(|n| *n -= 1);
                    comb.push(*k);
                    recurse(&mut ccs, target - k, comb, ret);
                    comb.pop();
                }
            }
        }
    }

    let mut cs = BTreeMap::new();

    for c in candidates.iter().filter(|c| **c <= target) {
        cs.entry(*c).and_modify(|n| *n += 1).or_insert(1i32);
    }

    let mut ret = vec![];

    recurse(&mut cs, target, &mut Vec::new(), &mut ret);

    ret
}

fn main() {
    println!("{:?}", combination_sum2(vec![10,1,2,7,6,1,5], 8));
}
