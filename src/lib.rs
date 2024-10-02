// use std::collections::HashSet;


// pub fn aux2(facets: &Vec<Vec<i32>>, leaders: &mut Vec<i32>, i: usize, t: &mut Vec<Vec<u64>>, k: usize, d: usize){
//     if k == d {
//         // check that leaders are fine
//         // every x of facets[i] should be a least one time leader

//         // apply modifications to t
//         aux(facets, i+1, t, d );
//     } else {
//         for &x in facets[i].iter(){
//             leaders.push(x);
//             aux2(facets, leaders, i, t, k+1, d);
//             leaders.pop();
//         }
//     }
// }


// pub fn aux(facets: &Vec<Vec<i32>>, i: usize, t: &mut Vec<Vec<u64>>, d: usize) {
//     let facet = &facets[i];
//     // aux2
// }

// pub fn dushnik_miller_dim2(
//     facets: &Vec<Vec<i32>>,
//     n: usize,
//     d: usize
// ) -> Option<Vec<Vec<i32>>> {
//     let delta: Vec<HashSet<i32>> = facets.into_iter().map(|v| v.into_iter().collect()).collect();

//     let mut t: Vec<Vec<u64>> = vec![vec![0; n]; d];

//     // aux(facets, 0, t, d);



//     None
// }