mod solution {
    use super::shortest_path;

    pub fn main(
        n: i32,
        edges: Vec<Vec<i32>>,
        threshold: i32)
     -> i32 {
        let n = n as usize;
        let dist = shortest_path::floyd_undirected(
            n,
            edges.iter().map(|edge| (
                edge[0] as _,
                edge[1] as _,
                edge[2] as _)));
        (0..n).rev().min_by_key(|&j| {
            (0..n)
                .filter(|&i| dist[i][j] <= threshold)
                .count()
        }).unwrap() as _
    }
}

mod shortest_path {
    #[inline]
    #[allow(dead_code)]
    #[allow(clippy::needless_range_loop)]
    pub fn floyd(
        n: usize,
        edges: impl Iterator<Item = (usize, usize, i32)>)
     -> Vec<Vec<i32>> {
        let mut dist = vec![vec![i32::MAX; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for (i, j, weight) in edges {
            dist[i][j] = weight;
            dist[j][i] = weight;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(
                        i32::saturating_add(
                            dist[i][k],
                            dist[k][j]));
                }
            }
        }
        dist
    }

    #[inline]
    #[allow(clippy::needless_range_loop)]
    pub fn floyd_undirected(
        n: usize,
        edges: impl Iterator<Item = (usize, usize, i32)>)
     -> Vec<Vec<i32>> {
        let mut dist = vec![vec![i32::MAX; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for (i, j, weight) in edges {
            dist[i][j] = weight;
            dist[j][i] = weight;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..i {
                    dist[i][j] = dist[i][j].min(
                        i32::saturating_add(
                            dist[i][k],
                            dist[k][j]));
                    dist[j][i] = dist[i][j];
                }
            }
        }
        dist
    }
}

fn main() {
    assert_eq!(solution::main(4, vec![
        vec![0, 1, 3],
        vec![1, 2, 1],
        vec![1, 3, 4],
        vec![2, 3, 1],
    ], 4), 3);
    assert_eq!(solution::main(5, vec![
        vec![0, 1, 2],
        vec![0, 4, 8],
        vec![1, 2, 3],
        vec![1, 4, 2],
        vec![2, 3, 1],
        vec![3, 4, 1],
    ], 2), 0);
}