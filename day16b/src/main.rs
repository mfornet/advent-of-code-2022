use std::collections::HashMap;

fn compute(
    node: usize,
    mask: usize,
    rem: i32,
    mask_map: &Vec<Option<i32>>,
    flows: &Vec<i32>,
    g: &Vec<Vec<usize>>,
    memo: &mut HashMap<(usize, usize, i32), i32>,
) -> i32 {
    if rem == 0 {
        return 0;
    }

    if let Some(val) = memo.get(&(node, mask, rem)) {
        return *val;
    }

    let mut ret = 0;

    if let Some(id) = mask_map[node] {
        if ((mask >> id) & 1) == 0 {
            ret = flows[node] * (rem - 1)
                + compute(node, mask | (1 << id), rem - 1, mask_map, flows, g, memo);
        }
    }

    for v in g[node].iter() {
        let cur = compute(*v, mask, rem - 1, mask_map, flows, g, memo);
        ret = std::cmp::max(ret, cur);
    }

    memo.insert((node, mask, rem), ret);
    ret
}

fn main() {
    let pat = regex::Regex::new(
        r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (((\w+(, )?))+)",
    )
    .unwrap();

    let mut nodes = HashMap::new();
    let mut flows = vec![];
    let mut graph = vec![];
    let mut mask_map = vec![];

    let mut mask_pos = 0;
    let mut non_zero = 0;

    for line in aoc::input_lines(file!()) {
        let cap = pat.captures(&line).unwrap();

        let source = cap.get(1).unwrap().as_str().to_string();

        if !nodes.contains_key(&source) {
            let n = nodes.len();
            nodes.insert(source.clone(), n);
            graph.push(vec![]);
            mask_map.push(None);
            flows.push(0);
        }

        let u = nodes.get(&source).copied().unwrap();

        let flow = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        flows[u] = flow;

        if flow != 0 {
            non_zero += 1;
            mask_map[u] = Some(mask_pos);
            mask_pos += 1;
        }

        let adj = cap.get(3).unwrap().as_str();

        for v in adj.split(", ") {
            if !nodes.contains_key(v) {
                let n = nodes.len();
                nodes.insert(v.to_string(), n);
                graph.push(vec![]);
                mask_map.push(None);
                flows.push(0);
            }

            let v = nodes.get(v).copied().unwrap();
            graph[u].push(v);
        }
    }

    let mut sub_solve = vec![0; 1 << non_zero];
    let big_mask = (1 << non_zero) - 1;

    let source = nodes.get("AA").copied().unwrap();
    let mut memo = Default::default();

    for mask in 0..(1 << non_zero) {
        let cur = compute(
            source,
            mask ^ big_mask,
            26,
            &mask_map,
            &flows,
            &graph,
            &mut memo,
        );
        sub_solve[mask] = cur;

        for i in 0..non_zero {
            if ((mask >> i) & 1) == 1 {
                sub_solve[mask] = std::cmp::max(sub_solve[mask], sub_solve[mask ^ (1 << i)]);
            }
        }
    }

    let mut res = 0;

    for mask in 0..(1 << non_zero) {
        let cur = sub_solve[mask] + sub_solve[big_mask ^ mask];

        if cur > res {
            res = cur;
        }
    }

    println!("{}", res);
}
