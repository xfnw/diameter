use std::collections::{BTreeMap, BTreeSet};

/// get the farthest node by walking from a node
///
/// will panic on invalid input, it is recommended to use a tree from
/// [`SpanningTree::nodes`] after verifying it is not empty
///
/// ```rust
/// let mut graph = diameter::SpanningTree::default();
/// graph.add_link("yip", "yap");
/// let (edges, names) = graph.tree();
/// let (farthest, distance) = diameter::get_farthest(0, edges);
///
/// assert_eq!(names[farthest], "yap");
/// assert_eq!(distance, 1);
/// ```
pub fn get_farthest(from: usize, edges: &[Vec<usize>]) -> (usize, usize) {
    let mut longest: (&usize, usize) = (&from, 0);
    let mut path: Vec<(&usize, usize)> = vec![(&from, 0)];
    let mut visited: BTreeSet<&usize> = BTreeSet::new();

    while let Some((current, i)) = path.pop() {
        // since we popped, no need to subtract 1 from length to
        // get number of hops
        let length = path.len();
        if length > longest.1 {
            longest = (current, length);
        }

        visited.insert(current);

        let connections = edges.get(*current).expect("nonexistent server referenced");

        if i < connections.len() {
            path.push((current, i + 1));

            let next = &connections[i];
            if !visited.contains(next) {
                path.push((next, 0));
            }
        }
    }

    (*longest.0, longest.1)
}

macro_rules! make_ids {
    (($($name:expr),*), $lookup:expr) => {
        ($({
            make_ids!($name, $lookup, {})
        },)*)
    };
    (($($name:expr),*), $lookup:expr, $nodenames:expr, $nodes:expr) => {
        ($({
            make_ids!($name, $lookup, {
                $nodenames.push($name.to_string());
                $nodes.push(vec![]);
            })
        },)*)
    };
    ($name:expr, $lookup:expr, $extra:expr) => {
        match $lookup.get($name) {
            Some(id) => *id,
            None => {
                let newid = $lookup.len();
                $extra
                $lookup.insert($name.to_string(), newid);
                newid
            }
        }
    };
}

#[deprecated(since = "0.3.0", note = "consider using SpanningTree instead")]
pub fn add_link(
    from: String,
    to: String,
    namelookup: &mut BTreeMap<String, usize>,
    servers: &mut BTreeMap<usize, Vec<usize>>,
) {
    let (from, to) = make_ids!((&from, &to), *namelookup);

    match servers.get_mut(&from) {
        Some(connections) => {
            connections.push(to);
        }
        None => {
            servers.insert(from, vec![to]);
        }
    }
    match servers.get_mut(&to) {
        Some(connections) => {
            connections.push(from);
        }
        None => {
            servers.insert(to, vec![from]);
        }
    }
}

#[deprecated(since = "0.3.0", note = "consider using SpanningTree instead")]
pub fn collect_servers(
    servers: BTreeMap<usize, Vec<usize>>,
    namelookup: BTreeMap<String, usize>,
) -> (Vec<Vec<usize>>, Vec<String>) {
    let servers: Vec<Vec<usize>> = servers.into_values().collect();

    let mut servernames: Vec<String> = vec!["".to_string(); namelookup.len()];
    for (name, id) in namelookup.into_iter() {
        servernames[id] = name;
    }

    (servers, servernames)
}

/// a representation of a spanning tree, an undirected graph without loops
#[derive(Default, Debug, Clone)]
pub struct SpanningTree {
    nodes: Vec<String>,
    lookup: BTreeMap<String, usize>,
    edges: Vec<Vec<usize>>,
}

impl SpanningTree {
    /// add an edge to the tree. any missing nodes will be automatically created.
    ///
    /// ```rust
    /// let mut graph = diameter::SpanningTree::default();
    /// let ids = graph.add_link("some.server", "other.server");
    ///
    /// assert_eq!(ids, (0, 1));
    /// println!("{:?}", graph);
    /// ```
    pub fn add_link(&mut self, from: &str, to: &str) -> (usize, usize) {
        let (from, to) = make_ids!((from, to), self.lookup, self.nodes, self.edges);

        if from != to {
            self.edges[from].push(to);
            self.edges[to].push(from);
        }

        (from, to)
    }

    /// calculate the diameter and two of the farthest nodes
    ///
    /// ```rust
    /// let mut graph = diameter::SpanningTree::default();
    ///
    /// assert_eq!(graph.diameter(), None);
    ///
    /// graph.add_link("yip", "yap");
    /// graph.add_link("yap", "yop");
    /// graph.add_link("yop", "yote");
    /// let (length, a, b) = graph.diameter().unwrap();
    ///
    /// assert_eq!(length, 3);
    /// assert_eq!(a, "yote");
    /// assert_eq!(b, "yip");
    /// ```
    pub fn diameter(&self) -> Option<(usize, &str, &str)> {
        if self.edges.is_empty() {
            return None;
        }

        let (node_a, _) = get_farthest(0, &self.edges);
        let (node_b, length) = get_farthest(node_a, &self.edges);

        Some((length, &self.nodes[node_a], &self.nodes[node_b]))
    }

    /// retrieve internal representation of the tree, and list of names
    ///
    /// useful for feeding into [`get_farthest`], although in most cases it is easier to use
    /// [`SpanningTree::diameter`]
    pub fn tree(&self) -> (&Vec<Vec<usize>>, &Vec<String>) {
        (&self.edges, &self.nodes)
    }

    /// retrieve the id number corresponding to a name
    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.lookup.get(name).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_farthest() {
        let servers: Vec<Vec<usize>> = vec![
            vec![1, 3],
            vec![0, 2],
            vec![1],
            vec![0, 4],
            vec![3, 5],
            vec![4],
        ];

        assert_eq!(get_farthest(0, &servers), (5, 3));
    }

    #[test]
    #[should_panic(expected = "nonexistent server referenced")]
    fn check_nonexist_server() {
        let servers: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![0], vec![0]];

        get_farthest(0, &servers);
    }
}
