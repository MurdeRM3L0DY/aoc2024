use std::collections::{HashMap, HashSet};

use indexmap::IndexSet;
use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::fold_many1,
    sequence::{separated_pair, terminated},
};
use petgraph::{Graph, Undirected, graph::NodeIndex};

fn parse(input: &str) -> nom::IResult<&str, Graph<&str, (), petgraph::Undirected>> {
    fold_many1(
        terminated(separated_pair(alpha1, tag("-"), alpha1), line_ending),
        || (Graph::new_undirected(), HashMap::new()),
        |(mut graph, mut idx_map), (a, b)| {
            let a_idx = *idx_map.entry(a).or_insert_with(|| graph.add_node(a));
            let b_idx = *idx_map.entry(b).or_insert_with(|| graph.add_node(b));

            graph.add_edge(a_idx, b_idx, ());

            (graph, idx_map)
        },
    )
    .map(|(graph, _)| graph)
    .parse(input)
}

fn part1(parsed: &Graph<&str, (), Undirected>) {
    let mut triangles = HashSet::new();
    let mut count = 0;

    for e in parsed.edge_indices() {
        let (a, b) = parsed.edge_endpoints(e).unwrap();
        let na = parsed.neighbors(a).collect::<HashSet<_>>();
        let nb = parsed.neighbors(b).collect::<HashSet<_>>();

        for &c in na.intersection(&nb) {
            let mut nodes = [a, b, c];
            nodes.sort();
            triangles.insert((nodes[0], nodes[1], nodes[2]));
        }
    }

    for &(a, b, c) in &triangles {
        if parsed[a].starts_with('t') || parsed[b].starts_with('t') || parsed[c].starts_with('t') {
            count += 1;
        }
    }

    println!("part1: {:?}", count);
}

fn pivot(
    p: &IndexSet<NodeIndex>,
    x: &IndexSet<NodeIndex>,
    graph: &Graph<&str, (), Undirected>,
) -> NodeIndex {
    *p.union(x)
        .max_by_key(|&&node| {
            graph
                .neighbors(node)
                .filter(|n| p.contains(n) || x.contains(n))
                .count()
        })
        .unwrap()
}

fn get_maximal_cliques(
    parsed: &Graph<&str, (), Undirected>,
    cr: IndexSet<NodeIndex>,
    p: IndexSet<NodeIndex>,
    mut x: IndexSet<NodeIndex>,
) -> Vec<IndexSet<NodeIndex>> {
    if p.is_empty() && x.is_empty() {
        return Vec::from_iter([cr]);
    }

    let mut r = Vec::new();

    let pvt = pivot(&p, &x, parsed);

    for v in p
        .difference(&parsed.neighbors(pvt).collect::<IndexSet<_>>())
        .copied()
    {
        let nv = parsed.neighbors(v).collect::<IndexSet<_>>();
        let mut cr = cr.iter().copied().collect::<IndexSet<_>>();
        cr.insert(v);

        r.extend(get_maximal_cliques(
            parsed,
            cr,
            p.intersection(&nv).copied().collect(),
            x.intersection(&nv).copied().collect(),
        ));

        x.insert(v);
    }

    r.sort_by_key(|i| i.len());

    r
}

fn part2(parsed: &Graph<&str, (), Undirected>) {
    let mcliques = get_maximal_cliques(
        parsed,
        IndexSet::new(),
        parsed.node_indices().collect::<IndexSet<_>>(),
        IndexSet::new(),
    );

    println!(
        "part2: {:?}",
        mcliques
            .last()
            .unwrap()
            .iter()
            .map(|&i| parsed[i])
            .sorted()
            .join(",")
    );
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day23.input");
    let (_, parsed) = parse(input)?;

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
