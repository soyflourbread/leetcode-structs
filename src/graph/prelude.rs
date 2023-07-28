// Given a vector of edges `[v_src, v_dest, weight]`,
//   returns an array of next vertices and weights for each vertex.
pub fn edges_to_graph(edges: Vec<Vec<i32>>, n: usize) -> [Vec<Vec<(usize, u32)>>; 2] {
    let mut ret_0 = vec![vec![]; n];
    let mut ret_1 = vec![vec![]; n];
    for v in edges {
        assert!(v.len() >= 3, "malformed input");
        let v0 = v[0] as usize;
        let v1 = v[1] as usize;
        let weight = v[2] as u32;

        ret_0[v0].push((v1, weight));
        ret_1[v1].push((v0, weight));
    }
    [ret_0, ret_1]
}

pub fn undirected_edges_to_graph(edges: Vec<Vec<i32>>, n: usize) -> Vec<Vec<(usize, u32)>> {
    let mut ret = vec![vec![]; n];
    for v in edges {
        assert!(v.len() >= 3, "malformed input");
        let v0 = v[0] as usize;
        let v1 = v[1] as usize;
        let weight = v[2] as u32;

        ret[v0].push((v1, weight));
        ret[v1].push((v0, weight));
    }
    ret
}
