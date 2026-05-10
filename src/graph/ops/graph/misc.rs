//! functions that has a graph among its arguments that output a value

use crate::errors::{PGMRustError, PGMRustResult};
use crate::graph::ops::edge::boolops::is_endvertice;
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node as NodeTrait;
use std::collections::HashMap;
use std::collections::HashSet;
use std::option::Option;

/// create an edge list representation of graph
/// for each node we register all the edges
pub fn to_adjacencylist<'a, N, E, G>(g: &'a G) -> HashMap<&str, Option<HashSet<&str>>>
where
    N: NodeTrait + 'a,
    E: EdgeTrait<N> + 'a,
    G: GraphTrait<N, E>,
{
    let mut elist: HashMap<&str, Option<HashSet<&str>>> = HashMap::new();
    for node in g.vertices() {
        let mut n_es: HashSet<&str> = HashSet::new();
        for edge in g.edges() {
            if is_endvertice(edge, node) {
                n_es.insert(edge.id());
            }
        }
        let nid = node.id();
        if n_es.is_empty() {
            elist.insert(nid, None);
        } else {
            elist.insert(nid, Some(n_es));
        }
    }
    elist
}

/// Obtain the adjacency matrix of the graph
/// # Description
/// Adjacency matrix contains information about the adjacency of vertices.
/// Its keys are vertex identifiers, and values are booleans.
/// # Args
/// - g: something that implements [Graph] trait.
/// # Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::ops::graph::misc::to_adjmat;
/// use pgm_rust::graph::traits::graph_obj::GraphObject;
/// use pgm_rust::graph::types::node::Node;
/// use std::collections::HashMap;
/// use std::collections::HashSet;
/// fn mk_node(n_id: &str) -> Node {Node::empty(n_id)}
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_edges(es: Vec<Edge<Node>>) -> HashSet<Edge<Node>> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
///
/// fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
///     let mut hs: HashSet<Node> = HashSet::new();
///     for n in ns {
///         hs.insert(mk_node(n));
///     }
///     hs
/// }
///
/// let a = mk_node("a");
/// let b = mk_node("b");
/// let f = mk_node("f");
/// let e = mk_node("e");
/// let ae = mk_uedge("a", "e", "ae");
/// let af = mk_uedge("a", "f", "af");
/// let ef = mk_uedge("e", "f", "ef");
/// let nset = mk_nodes(vec!["a", "b", "f", "e"]);
/// let h1 = HashMap::new();
/// let h2 = mk_edges(vec![ae, af, ef]);
/// let g1 = Graph::new("g1".to_string(), h1, nset, h2);
/// let mut comp = HashMap::new();
/// comp.insert((b.id(), b.id()), false);
/// comp.insert((b.id(), e.id()), false);
/// comp.insert((b.id(), f.id()), false);
/// comp.insert((b.id(), a.id()), false);
/// comp.insert((e.id(), b.id()), false);
/// comp.insert((e.id(), e.id()), false);
/// comp.insert((e.id(), f.id()), true);
/// comp.insert((e.id(), a.id()), true);
/// comp.insert((f.id(), b.id()), false);
/// comp.insert((f.id(), e.id()), true);
/// comp.insert((f.id(), f.id()), false);
/// comp.insert((f.id(), a.id()), true);
/// comp.insert((a.id(), b.id()), false);
/// comp.insert((a.id(), e.id()), true);
/// comp.insert((a.id(), f.id()), true);
/// comp.insert((a.id(), a.id()), false);
/// let amat = to_adjmat(&g1);
/// amat == comp; // true
/// ```
pub fn to_adjmat<'a, N, E, G>(g: &'a G) -> HashMap<(&'a String, &'a String), bool>
where
    N: NodeTrait + 'a,
    E: EdgeTrait<N> + 'a,
    G: GraphTrait<N, E>,
{
    //
    let mut adjmat = HashMap::new();
    for e in g.edges() {
        let n1 = e.start();
        let n2 = e.end();
        let n1_id = n1.id();
        let n2_id = n2.id();
        adjmat.insert((n1_id, n2_id), true);
        adjmat.insert((n2_id, n1_id), true);
    }
    for n1 in g.vertices() {
        for n2 in g.vertices() {
            let n1_id = n1.id();
            let n2_id = n2.id();
            if !adjmat.contains_key(&(n1_id, n2_id)) {
                adjmat.insert((n1_id, n2_id), false);
                adjmat.insert((n2_id, n1_id), false);
            }
        }
    }
    adjmat
}

/// obtain graph object using its identifier
pub fn by_id<'a, N, E, G, T, F>(g: &'a G, id: &str, f: F) -> PGMRustResult<&'a T>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
    T: GraphObject,
    F: Fn(&'a G) -> HashSet<&'a T>,
{
    for h in f(g) {
        if h.id() == id {
            return Ok(h);
        }
    }
    Err(PGMRustError::NotInGraph(id.to_string(), g.to_string()))
}

/// Get subgraph using given vertices
/// # Description
/// We extract the subgraph using the provided node set.
///
/// # Args
/// - g: something that implements [Graph] trait.
/// - ns: a set of things that implement [Node] trait
/// - edge_policy: defines how to handle edges given a node. By default, we
/// conserve edges whose incident nodes are a subset of `ns`
pub fn get_subgraph_by_vertices<'a, G, N, E, F>(
    g: &'a G,
    ns: HashSet<&N>,
    edge_policy: Option<F>,
) -> (HashSet<&'a N>, HashSet<&'a E>)
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
    F: Fn(&'a E, &HashSet<&N>) -> bool,
{
    let policy = |e: &'a E, vs: &HashSet<&N>| -> bool {
        match &edge_policy {
            Some(p) => p(e, vs),
            None => {
                let n1 = e.start();
                let n2 = e.end();
                let mut n1_c = false;
                let mut n2_c = false;
                for v in vs {
                    let vid = v.id();
                    if vid == n1.id() {
                        n1_c = true;
                    }
                    if vid == n2.id() {
                        n2_c = true;
                    }
                }
                n1_c && n2_c
            }
        }
    };
    let mut eset = HashSet::new();
    for e in g.edges() {
        if policy(e, &ns) {
            eset.insert(e);
        }
    }
    let vset: HashSet<&String> = ns.iter().map(|&n| -> &String { n.id() }).collect();
    let mut nset = HashSet::new();
    for n in g.vertices() {
        if vset.contains(n.id()) {
            nset.insert(n);
        }
    }
    (nset, eset)
}

/// Check whether a given mapping is a valid graph isomorphism between two graphs.
/// This is a verifier function, where the caller takes responsibility for knowing
/// (or guessing) the correspondence. check_isomorphism just confirms it. 
///
/// # Description
/// Given two graphs `g1` and `g2` and a vertex mapping `f`, returns `true` if and only if
/// `f` is a bijection from `V(g1)` to `V(g2)` that preserves adjacency (and non-adjacency).
/// Edge direction is ignored, consistent with the rest of the library.
///
/// Formally, `f` is a valid isomorphism when:
/// - `f` is injective: distinct nodes in `g1` map to distinct nodes in `g2`
/// - `f` is surjective: every node in `g2` is the image of some node in `g1`
/// - Edge-preserving: for every edge `{s, t}` in `g1`, there exists an edge `{f(s), f(t)}` in `g2`
///
/// # Args
/// - `g1`: first graph
/// - `g2`: second graph
/// - `f`: closure mapping a node from `g1` to a node in `g2`
///
/// # Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::graph::misc::check_isomorphism;
/// use std::collections::HashSet;
/// use std::collections::HashMap;
///
/// // g1: triangle on {a, b, c}
/// let e1 = Edge::empty("e1", EdgeType::Undirected, "a", "b");
/// let e2 = Edge::empty("e2", EdgeType::Undirected, "b", "c");
/// let e3 = Edge::empty("e3", EdgeType::Undirected, "a", "c");
/// let g1 = Graph::from_edge_node_set(
///     HashSet::from([e1, e2, e3]),
///     HashSet::from([Node::empty("a"), Node::empty("b"), Node::empty("c")]),
/// );
///
/// // g2: triangle on {x, y, z}
/// let e4 = Edge::empty("e4", EdgeType::Undirected, "x", "y");
/// let e5 = Edge::empty("e5", EdgeType::Undirected, "y", "z");
/// let e6 = Edge::empty("e6", EdgeType::Undirected, "x", "z");
/// let g2 = Graph::from_edge_node_set(
///     HashSet::from([e4, e5, e6]),
///     HashSet::from([Node::empty("x"), Node::empty("y"), Node::empty("z")]),
/// );
///
/// // f: a->x, b->y, c->z
/// let f = |n: &Node| -> Node {
///     match n.id().as_str() {
///         "a" => Node::empty("x"),
///         "b" => Node::empty("y"),
///         _   => Node::empty("z"),
///     }
/// };
/// assert!(check_isomorphism(&g1, &g2, f));
/// ```
pub fn check_isomorphism<N, E, G, F>(g1: &G, g2: &G, f: F) -> bool
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
    F: Fn(&N) -> N,
{
    // 1. Size guards
    if g1.vertices().len() != g2.vertices().len() {
        return false;
    }
    if g1.edges().len() != g2.edges().len() {
        return false;
    }

    // 2. Apply f to all nodes in g1
    // mapped is the list of output nodes after running every vertex of g1 through f
    let mapped: Vec<N> = g1.vertices().iter().map(|n| f(n)).collect();

    // 3. Injectivity: all mapped IDs must be distinct
    // two different inputs should not map to the same output
    let mapped_ids: HashSet<&str> = mapped.iter().map(|n| n.id()).collect();
    if mapped_ids.len() != mapped.len() {
        return false;
    }

    // 4. Surjectivity: mapped IDs must equal V(g2) IDs
    let g2_ids: HashSet<&str> = g2.vertices().iter().map(|n| n.id()).collect();
    if mapped_ids != g2_ids {
        return false;
    }

    // 5. Edge preservation: build a set of (start_id, end_id) pairs from g2 (both orderings)
    let g2_edge_pairs: HashSet<(&str, &str)> = g2
        .edges()
        .iter()
        .flat_map(|e| {
            let s = e.start().id();
            let t = e.end().id();
            [(s, t), (t, s)]
        })
        .collect();

    // For each edge in g1, check that the mapped edge exists in g2
    for e in g1.edges() {
        let fs = f(e.start());
        let ft = f(e.end());
        if !g2_edge_pairs.contains(&(fs.id(), ft.id())) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::types::edge::Edge;
    use crate::graph::types::edgetype::EdgeType;
    use crate::graph::types::graph::Graph;
    use crate::graph::types::node::Node;
    use std::collections::HashMap;
    use std::collections::HashSet;

    fn mk_node(n_id: &str) -> Node {
        Node::empty(n_id)
    }
    fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
        let mut hs: HashSet<Node> = HashSet::new();
        for n in ns {
            hs.insert(mk_node(n));
        }
        hs
    }
    fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
        Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
    }
    fn mk_edges(es: Vec<Edge<Node>>) -> HashSet<Edge<Node>> {
        let mut hs = HashSet::new();
        for e in es {
            hs.insert(e);
        }
        hs
    }
    fn mk_g1() -> Graph<Node, Edge<Node>> {
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        let e3 = mk_uedge("n2", "n4", "e3");
        let nset = mk_nodes(vec!["n1", "n2", "n3", "n4", "n5"]);
        let h1 = HashMap::new();
        let h2 = mk_edges(vec![e1, e2, e3]);
        Graph::new("g1".to_string(), h1, nset, h2)
    }

    fn mk_refset(es: Vec<&str>) -> HashSet<&str> {
        let mut ns: HashSet<&str> = HashSet::new();
        for e in es {
            ns.insert(e);
        }
        ns
    }

    #[test]
    fn test_to_adjacencylist() {
        let g = mk_g1();
        let alst = to_adjacencylist(&g);
        let n4_ns = mk_refset(vec!["e3"]);
        //
        let n3_ns = mk_refset(vec!["e1", "e2"]);
        //
        let n2_ns = mk_refset(vec!["e2", "e3"]);
        //
        let n1_ns = mk_refset(vec!["e1"]);
        //
        let mut comp = HashMap::new();
        comp.insert("n5", None);
        comp.insert("n4", Some(n4_ns));
        comp.insert("n3", Some(n3_ns));
        comp.insert("n2", Some(n2_ns));
        comp.insert("n1", Some(n1_ns));
        assert_eq!(comp, alst);
    }

    #[test]
    fn test_to_adjmat() {
        let a = mk_node("a");
        let b = mk_node("b");
        let f = mk_node("f");
        let e = mk_node("e");
        let ae = mk_uedge("a", "e", "ae");
        let af = mk_uedge("a", "f", "af");
        let ef = mk_uedge("e", "f", "ef");
        let nset = mk_nodes(vec!["a", "b", "f", "e"]);
        let h1 = HashMap::new();
        let h2 = mk_edges(vec![ae, af, ef]);
        let g1 = Graph::new("g1".to_string(), h1, nset, h2);
        let mut comp = HashMap::new();
        comp.insert((b.id(), b.id()), false);
        comp.insert((b.id(), e.id()), false);
        comp.insert((b.id(), f.id()), false);
        comp.insert((b.id(), a.id()), false);
        comp.insert((e.id(), b.id()), false);
        comp.insert((e.id(), e.id()), false);
        comp.insert((e.id(), f.id()), true);
        comp.insert((e.id(), a.id()), true);
        comp.insert((f.id(), b.id()), false);
        comp.insert((f.id(), e.id()), true);
        comp.insert((f.id(), f.id()), false);
        comp.insert((f.id(), a.id()), true);
        comp.insert((a.id(), b.id()), false);
        comp.insert((a.id(), e.id()), true);
        comp.insert((a.id(), f.id()), true);
        comp.insert((a.id(), a.id()), false);
        let amat = to_adjmat(&g1);
        assert_eq!(amat, comp);
    }

    #[test]
    fn test_get_subgraph_by_vertices_default_edge_policy() {
        let g1 = mk_g1();
        let n1 = mk_node("n1");
        let n2 = mk_node("n2");
        let n4 = mk_node("n4");
        let mut nrefset = HashSet::new();
        nrefset.insert(&n1);
        nrefset.insert(&n2);
        nrefset.insert(&n4);
        let nrefset2 = nrefset.clone();
        let mut erefset = HashSet::new();
        let e1 = mk_uedge("n2", "n4", "e3");
        erefset.insert(&e1);
        // let opt: Option<dyn Fn(&Edge, &HashSet<&Node>) -> bool> = None;
        let opt: Option<Box<dyn Fn(&Edge<Node>, &HashSet<&Node>) -> bool>> = None;
        // let opt = None;
        let result: (HashSet<&Node>, HashSet<&Edge<Node>>) =
            get_subgraph_by_vertices(&g1, nrefset, opt);
        let (nodes, edges) = result;
        assert_eq!(nodes, nrefset2);

        //
        assert_eq!(edges, erefset);
    }

    #[test]
    fn test_get_subgraph_by_vertices_inclusive_edge_policy() {
        let g1 = mk_g1();
        let n1 = mk_node("n1");
        let n3 = mk_node("n3");
        let mut nrefset = HashSet::new();
        nrefset.insert(&n1);
        nrefset.insert(&n3);
        let nrefset2 = nrefset.clone();
        let mut erefset = HashSet::new();
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        erefset.insert(&e1);
        erefset.insert(&e2);
        // let opt: Option<dyn Fn(&Edge, &HashSet<&Node>) -> bool> = None;
        let policy = |e: &Edge<Node>, vs: &HashSet<&Node>| -> bool {
            let n1 = e.start();
            let n2 = e.end();
            let mut n1_c = false;
            let mut n2_c = false;
            for v in vs {
                let vid = v.id();
                if vid == n1.id() {
                    n1_c = true;
                }
                if vid == n2.id() {
                    n2_c = true;
                }
            }
            n1_c || n2_c
        };

        let opt = Some(policy);
        // let opt = None;
        let result: (HashSet<&Node>, HashSet<&Edge<Node>>) =
            get_subgraph_by_vertices(&g1, nrefset, opt);
        let (nodes, edges) = result;
        assert_eq!(nodes, nrefset2);

        //
        assert_eq!(edges, erefset);
    }

    // --- check_isomorphism tests ---

    fn mk_triangle(prefix: &str) -> Graph<Node, Edge<Node>> {
        let a = mk_node(&format!("{prefix}a"));
        let b = mk_node(&format!("{prefix}b"));
        let c = mk_node(&format!("{prefix}c"));
        let e1 = mk_uedge(&format!("{prefix}a"), &format!("{prefix}b"), &format!("{prefix}e1"));
        let e2 = mk_uedge(&format!("{prefix}b"), &format!("{prefix}c"), &format!("{prefix}e2"));
        let e3 = mk_uedge(&format!("{prefix}a"), &format!("{prefix}c"), &format!("{prefix}e3"));
        let nset = HashSet::from([a, b, c]);
        let eset = mk_edges(vec![e1, e2, e3]);
        Graph::new("g".to_string(), HashMap::new(), nset, eset)
    }

    #[test]
    fn test_check_isomorphism_valid() {
        // g1: triangle {a,b,c}, g2: triangle {x,y,z}
        // f: a->x, b->y, c->z is a valid isomorphism
        let g1 = mk_triangle("g1_");
        let g2 = mk_triangle("g2_");
        let f = |n: &Node| -> Node {
            match n.id().trim_start_matches("g1_") {
                "a" => mk_node("g2_a"),
                "b" => mk_node("g2_b"),
                _   => mk_node("g2_c"),
            }
        };
        assert!(check_isomorphism(&g1, &g2, f));
    }

    #[test]
    fn test_check_isomorphism_non_injective() {
        // f maps two distinct nodes to the same target — not injective
        let g1 = mk_triangle("g1_");
        let g2 = mk_triangle("g2_");
        let f = |_n: &Node| -> Node { mk_node("g2_a") }; // all map to same node
        assert!(!check_isomorphism(&g1, &g2, f));
    }

    #[test]
    fn test_check_isomorphism_edge_breaking() {
        // g1: triangle {a,b,c} with edges a-b, b-c, a-c
        // g2: path   {x,y,z} with edges x-y, y-z only (missing x-z)
        let g1 = mk_triangle("g1_");
        let e4 = mk_uedge("g2_x", "g2_y", "g2_e1");
        let e5 = mk_uedge("g2_y", "g2_z", "g2_e2");
        // deliberately only 2 edges — size guard will catch this
        let g2: Graph<Node, Edge<Node>> = Graph::new(
            "g2".to_string(),
            HashMap::new(),
            HashSet::from([mk_node("g2_x"), mk_node("g2_y"), mk_node("g2_z")]),
            mk_edges(vec![e4, e5]),
        );
        let f = |n: &Node| -> Node {
            match n.id().trim_start_matches("g1_") {
                "a" => mk_node("g2_x"),
                "b" => mk_node("g2_y"),
                _   => mk_node("g2_z"),
            }
        };
        assert!(!check_isomorphism(&g1, &g2, f));
    }

    #[test]
    fn test_check_isomorphism_size_mismatch() {
        // g1 has 3 nodes, g2 has 4 nodes — trivially not isomorphic
        let g1 = mk_triangle("g1_");
        let g2 = mk_g1(); // 5 nodes, 3 edges
        let f = |n: &Node| -> Node { mk_node(n.id()) };
        assert!(!check_isomorphism(&g1, &g2, f));
    }

    fn mk_path(prefix: &str) -> Graph<Node, Edge<Node>> {
    // a-b-c-d
    let a = mk_node(&format!("{prefix}a"));
    let b = mk_node(&format!("{prefix}b"));
    let c = mk_node(&format!("{prefix}c"));
    let d = mk_node(&format!("{prefix}d"));
    let e1 = mk_uedge(&format!("{prefix}a"), &format!("{prefix}b"), &format!("{prefix}e1"));
    let e2 = mk_uedge(&format!("{prefix}b"), &format!("{prefix}c"), &format!("{prefix}e2"));
    let e3 = mk_uedge(&format!("{prefix}c"), &format!("{prefix}d"), &format!("{prefix}e3"));
    let nset = HashSet::from([a, b, c, d]);
    let eset = mk_edges(vec![e1, e2, e3]);
    Graph::new("g".to_string(), HashMap::new(), nset, eset)
    }

    fn mk_star(prefix: &str) -> Graph<Node, Edge<Node>> {
        // hub connected to 3 leaves: hub-l1, hub-l2, hub-l3
        let hub = mk_node(&format!("{prefix}hub"));
        let l1  = mk_node(&format!("{prefix}l1"));
        let l2  = mk_node(&format!("{prefix}l2"));
        let l3  = mk_node(&format!("{prefix}l3"));
        let e1 = mk_uedge(&format!("{prefix}hub"), &format!("{prefix}l1"), &format!("{prefix}e1"));
        let e2 = mk_uedge(&format!("{prefix}hub"), &format!("{prefix}l2"), &format!("{prefix}e2"));
        let e3 = mk_uedge(&format!("{prefix}hub"), &format!("{prefix}l3"), &format!("{prefix}e3"));
        let nset = HashSet::from([hub, l1, l2, l3]);
        let eset = mk_edges(vec![e1, e2, e3]);
        Graph::new("g".to_string(), HashMap::new(), nset, eset)
    }

    #[test]
    fn test_check_isomorphism_wrong_structure() {
        // path and star: same size (4 nodes, 3 edges), but different structure
        let g1 = mk_path("g1_");
        let g2 = mk_star("g2_");
        // f: a->hub, b->l1, c->l2, d->l3
        // bijective, but b-c in g1 maps to l1-l2, which doesn't exist in g2
        let f = |n: &Node| -> Node {
            match n.id().trim_start_matches("g1_") {
                "a"  => mk_node("g2_hub"),
                "b"  => mk_node("g2_l1"),
                "c"  => mk_node("g2_l2"),
                _    => mk_node("g2_l3"),
            }
        };
        assert!(!check_isomorphism(&g1, &g2, f));
    }
}
