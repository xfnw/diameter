use std::collections::{BTreeMap, BTreeSet};

pub fn get_farthest(from: usize, servers: &[Vec<usize>]) -> (usize, usize) {
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

        let connections = servers
            .get(*current)
            .expect("nonexistent server referenced");

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
            match $lookup.get(&$name) {
                Some(id) => *id,
                None => {
                    let newid = $lookup.len();
                    $lookup.insert($name, newid);
                    newid
                }
            }
        },)*)
    };
}

pub fn add_link(
    from: String,
    to: String,
    namelookup: &mut BTreeMap<String, usize>,
    servers: &mut BTreeMap<usize, Vec<usize>>,
) {
    let (from, to) = make_ids!((from, to), *namelookup);

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
