// fn main() {
//     let faces: Vec<Vec<i32>> = vec![
//         vec![0,1,2,5],
//         vec![0,1,3,4],
//         vec![0,2,3,4],
//         vec![1,2,3,4]
//     ];

//     let faces2: Vec<Vec<i32>> = vec![
//         vec![0,1,2,6],
//         vec![0,1,4,5],
//         vec![0,1,4,6],
//         vec![0,2,3,5],
//         vec![0,2,5,6],
//         vec![1,2,3,4],
//         vec![1,2,3,6],
//         vec![3,4,5]
//     ];

//     // let complex3: Vec<Vec<i32>> = vec![
//     //     vec![0,1,3,6],
//     //     vec![0,1,6,7],
//     //     vec![0,2,3,7],
//     //     vec![0,2,5,7],
//     //     vec![0,3,4],
//     //     vec![0,4,5,7],
//     //     vec![1,2,3,5],
//     //     vec![1,2,5,6],
//     //     vec![1,3,4],
//     //     vec![1,4,6,7],
//     //     vec![2,3,4],
//     //     vec![2,4,5,6],
//     //     vec![4,5,6,7]
//     // ];

//     let complex3: Vec<Vec<i32>> = vec![
//         vec![0,1,3,4],
//         vec![0,1,3,6],
//         vec![0,1,4,7],
//         vec![0,1,6,7],
//         vec![0,2,3,4],
//         vec![0,2,3,7],
//         vec![0,2,4,5],
//         vec![0,2,5,7],
//         vec![0,4,5,7],
//         vec![1,2,3,4],
//         vec![1,2,3,5],
//         vec![1,2,4,6],
//         vec![1,2,5,6],
//         vec![1,4,6,7],
//         vec![2,4,5,6],
//         vec![4,5,6,7]
//     ];

//     // DM 5
//     let complex4 = vec![
//         vec![0,1,5,6],
//         vec![0,2,4,6],
//         vec![0,3,4,6],
//         vec![1,2,4,5],
//         vec![1,3,5,6],
//         vec![2,3,4,5],
//         vec![3,4,5,6]
//     ];

//     println!("{:?}", dushnik_miller_dim(complex3, 4, None));


// }



use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <filepath> <number>", args[0]);
        std::process::exit(1);
    }

    let number_str = &args[2];

    // Parse the number
    let dim: usize = match number_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number argument");
            std::process::exit(1);
        }
    };

    let file_path = &args[1];
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut data_map: HashMap<String, i32> = HashMap::new();
    let mut reverse_data_map: HashMap<i32, String> = HashMap::new();
    let mut facets = Vec::new();
    let mut index = 0;

    // Parse the file
    // Lines starting with a # are skipped
    // Facets elements are separated by a space " "
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("#"){
            continue;
        }
        let mut facet: Vec<i32> = Vec::new();
        for s in line.split_whitespace() {
            if let Some(id) = data_map.get(s) {
                let id2: i32 = (*id).try_into().unwrap();
                facet.push(id2);
            } else {
                data_map.insert(s.to_string(), index);
                reverse_data_map.insert(index, s.to_string());
                facet.push(index);
                index += 1;
            }
        }
        facets.push(facet);

    }
    println!("data_map: {data_map:?}");
    println!("reverse: {reverse_data_map:?}");

    if let Some(repre) = dushnik_miller_dim(facets, dim, None) {
        println!("Dim {dim} OK");
        println!("{repre:?}");
        for line in repre {
            for x in line {
                if let Some(id) = reverse_data_map.get(&x) {
                    print!("{id} ");
                }
            }
            println!("");
        }

    } else {
        println!("The complex is not of dimension {dim}");
    }



}






pub fn move_right(t: &mut [i32], i: usize, j: usize) {
    let x = t[i];
    for k in i+1..=j {
        t[k-1] = t[k];
    }
    t[j] = x;
}


pub fn move_left(t: &mut [i32], i: usize, j: usize) {
    let x = t[i];
    for k in (j..i).rev() {
        t[k +  1] = t[k];
    }
    t[j] = x;
}



pub fn check_inclusion(
    faces: &Vec<HashSet<i32>>,
    rep: &Vec<Vec<i32>>,
) -> Option<(usize, i32)> {
    let n = rep[0].len();
    let d = rep.len();
    for (index, face) in faces.iter().enumerate() {
        let mut dominating_elts = HashSet::new();
        for i in  0..d {
            for j in (0..n).rev() {
                dominating_elts.insert(rep[i][j]);
                if face.contains(&rep[i][j]) {
                    break;
                }
            }
        }
        if dominating_elts.len() < n {
            for j in  0..n {
                if !dominating_elts.contains(&rep[0][j]) {
                    return Some((index, rep[0][j]));
                }
            }
        }
    }
    None
}



fn aux_dm(
    delta: &Vec<HashSet<i32>>,
    inserted_elts: &mut HashSet<i32>,
    todo: &mut Vec<i32>,
    d: usize,
    rep: &mut Vec<Vec<i32>>,
) -> bool {

    if let Some(v) = todo.pop() {
        inserted_elts.insert(v);
        // Insert v at the beginning of each order
        for i in  0..d {
            rep[i].insert(0, v);
        }
    
        let m = rep[d-1].len();
        let mut pos = vec![0; d];


        loop{

            if let Some((face_index, non_dominating_elt)) = check_inclusion(delta, rep) {
                let face = &delta[face_index];

                if v == non_dominating_elt {
                    // Move right v just after the max element of face in rep[d-1]
                    for i in (0..m).rev() {
                        if face.contains(&rep[d-1][i]) {
                            move_right(&mut rep[d-1], pos[d-1], i);
                            pos[d-1] = i;
                            break;
                        }
                    }
                } else {
                    // Compute the last order where y < v (y = nonDominatingElt)
                    let mut i = d-1;
                    loop {
                        if let Some(j) = rep[i].iter().position(|&x| x == non_dominating_elt) {
                            if j >= pos[i] {
                                break;
                            }
                        }
                        if i == 0 {
                            break;
                        }
                        i -=  1;
                    }

                    let mut is_maximal = true;
                    for j in (0..i).rev() {
                        if pos[j] < m-1 {
                            move_right(&mut rep[j], pos[j], pos[j]+1);
                            pos[j] +=  1;
                            is_maximal = false;
                            break;
                        } else {
                            move_left(&mut rep[j], pos[j],  0);
                            pos[j] =  0;
                        }
                    }
                    if is_maximal {
                        // Clean
                        inserted_elts.remove(&v);
                        for j in  0..d {
                            rep[j].remove(pos[j]);
                        }
                        todo.push(v);
                        return false; 
                    } else {
                        for j in i..d {
                            move_left(&mut rep[j], pos[j],  0);
                            pos[j] =  0;
                        }
                    }
                }
            } else {
                if aux_dm(delta, inserted_elts, todo, d, rep){
                    return true;
                } else {
                    let mut is_maximal = true;
                    for i in (0..d).rev() {
                        if pos[i] == m -  1 {
                            move_left(&mut rep[i], pos[i],  0);
                            pos[i] =  0;
                        } else {
                            move_right(&mut rep[i], pos[i], pos[i] +  1);
                            pos[i] +=  1;
                            is_maximal = false;
                            break;
                        }
                    }
                    if is_maximal {
                        // Clean
                        inserted_elts.remove(&v);
                        for i in  0..d {
                            rep[i].remove(pos[i]);
                        }
                        todo.push(v);
                        return false;
                    }
                }
            }

        }


    } else {
        // Finito: a representation has been found
        return true;
    }


}





pub fn dushnik_miller_dim(
    faces: Vec<Vec<i32>>,
    d: usize,
    order: Option<Vec<i32>>,
) -> Option<Vec<Vec<i32>>> {
    let delta: Vec<HashSet<i32>> = faces.into_iter().map(|v| v.into_iter().collect()).collect();

    let mut vertices = HashSet::new();
    for face in &delta {
        for elt in face {
            vertices.insert(*elt);
        }
    }

    // Initialize an empty d-representation
    let mut rep: Vec<Vec<i32>> = vec![Vec::new(); d];

    let mut inserted_elts = HashSet::new();
    let mut todo: Vec<i32> = vertices.into_iter().collect();

    if let Some(ref order) = order {
        todo = order.clone().into_iter().collect();
    }

    println!("{:?}", todo);

    if aux_dm(&delta, &mut inserted_elts, &mut todo, d, &mut rep){
        return Some(rep);
    } else {
        None
    }
}