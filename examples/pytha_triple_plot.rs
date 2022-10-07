/*
    绘制毕达哥拉斯三元组点
*/
extern crate comp_number_theory;

use comp_number_theory::base::pythagorean_triples::pythagorean_triples;
use plotters::prelude::*;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_drawing_area =
        BitMapBackend::new("images/pytha_triple.png", (300, 200)).into_drawing_area();
    // And we can split the drawing area into 3x3 grid
    let child_drawing_areas = root_drawing_area.split_evenly((5000, 5000));
    // Then we fill the drawing area with different color
    // for (area, color) in child_drawing_areas.into_iter().zip(0..) {
    //     let (row, col, _) = pythagorean_triples(, n)
    //     if (row.pow(2) + col.pow(2)){
    //         area.fill("Red")?;
    //     }
        
    // }
    for per_vec in (0..50).permutations(2) {
        let (row, col, _) = pythagorean_triples(per_vec[0], per_vec[1]);
        // println!("{}, {}, {}", per_vec[0], per_vec[1], row * 5000 + col);
        if row > 0 {
            child_drawing_areas[(row * 5000 + col) as usize].fill(&WHITE)?;
        }
        
    }
    root_drawing_area.present()?;
    Ok(())
}