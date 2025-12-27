use erebus_vector::prelude::*;

fn main() {

    let v = Vector::<f64>::from_vec(vec![1.0, 4.0, 9.0, 22.0, 35.0, 50.0]).unwrap();
    let gm_vec = v.geometric_mean();
    println!("Vector geometric mean: {}", gm_vec);

    let vd = VectorData::<f64>::from_vec(
        vec![1.0, 4.0, 9.0, 22.0, 35.0, 50.0],
        bitvec::bitvec![1, 1, 1,1,1,1]
    ).unwrap();
    let gm_vd = vd.geometric_mean();
    println!("VectorData geometric mean: {}", gm_vd);

}