use itertools::Itertools;

pub trait Node: std::fmt::Debug {
    fn children(&self) -> Vec<Self>
    where
        Self: Sized;

    fn score(&self) -> f64;

    fn real_score(&self) -> u64;
}

pub fn beam_search(source: impl Node, beam_width: u64) -> u64 {
    let mut beam = vec![source];
    let mut best = 0;
    let mut best_expected = 0f64;

    while !beam.is_empty() {
        let mut next_beam = beam
            .into_iter()
            .flat_map(|node| node.children().into_iter().map(|node| (node.score(), node)))
            .collect_vec();

        for (score, node) in &next_beam {
            if *score > best_expected {
                best_expected = *score;
            }

            let score = node.real_score();

            if score > best {
                best = score;
            }
        }

        next_beam.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        next_beam.truncate(beam_width as usize);

        beam = next_beam.into_iter().map(|(_, node)| node).collect_vec();
    }

    best
}
