use std::time::Instant;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::io::{Write, BufWriter};

pub fn f(x: f32, y: f32) -> f32 {
    return 2.0 * (x * x - x + 1.0) + 2.0 * (y * y - y + 1.0);
}

pub fn compare_offense(object_vector: &Vec<Vec<f32>>, compare_vec: &Vec<Vec<f32>>) -> f32 {
    let mut max_value = 0.0 as f32;
    let n = object_vector.len();

    for i in 0..n {
        for j in 0..n {
            max_value = max_value.max((compare_vec[i][j] - object_vector[i][j]).abs());
        }
    }

    return max_value;
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    // Threads number
    ThreadPoolBuilder::new()
        .num_threads(12)
        .build_global()
        .unwrap();

    let n = 128;
    let u = (n - 1) as f32;
    let h = 1.0 / u;
    let steps = 500000;

    let mut vec = vec![vec![0.0; n]; n];
    let mut p_vec = vec![vec![0.0; n]; n];

    for i in 0..n {
        let x = i as f32 / u;
        let val = x * x - x + 1.0;
        vec[i][0] = val;
        vec[i][n - 1] = val;
        vec[0][i] = val;
        vec[n - 1][i] = val;
    }

    for i in 0..n {
        for j in 0..n {
            let x = i as f32 / u;
            let y = j as f32 / u;
            p_vec[i][j] = (x * x - x + 1.0) * (y * y - y + 1.0);
        }
    }

    let mut vec_copy = vec.clone();
    let mut counts = 0;

    for _step in 0..steps {
        counts += 1;

        // Red cells
        vec_copy
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, row)| {
                if i == 0 || i == n - 1 {
                    return;
                }

                for j in 1..n - 1 {
                    if (i + j) % 2 == 0 {
                        let x = i as f32 / u;
                        let y = j as f32 / u;
                        row[j] = 0.25
                            * (vec[i - 1][j]
                                + vec[i + 1][j]
                                + vec[i][j - 1]
                                + vec[i][j + 1]
                                - h * h * f(x, y));
                    }
                }
            });

        vec = vec_copy.clone();

        // Black cells
        vec_copy
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, row)| {
                if i == 0 || i == n - 1 {
                    return;
                }
                for j in 1..n - 1 {
                    if (i + j) % 2 == 1 {
                        let x = i as f32 / u;
                        let y = j as f32 / u;
                        row[j] = 0.25
                            * (vec[i - 1][j]
                                + vec[i + 1][j]
                                + vec[i][j - 1]
                                + vec[i][j + 1]
                                - h * h * f(x, y));
                    }
                }
            });
        vec = vec_copy.clone();

        if counts % 2000 == 0 {
            let difference = compare_offense(&p_vec, &vec_copy);
            println!(
                "{} iteration, max difference between real and close result is: {}",
                counts, difference
            );
            if difference.abs() <= 0.001 {
                break;
            }
        }
    }

    let file = File::create("u_result.txt")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "x\ty\tu(x,y)")?;

    for i in 0..n {
        for j in 0..n {
            let x = i as f32 / u;
            let y = j as f32 / u;
            writeln!(writer, "{:.5}\t{:.5}\t{:.8}", x, y, vec_copy[i][j])?;
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed in main loop: {:?}", duration);
    println!("done");

    Ok(())
}
