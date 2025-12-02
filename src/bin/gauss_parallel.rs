use std::env;

use std::time::Instant;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::io::{Write, BufWriter};

pub fn f(x: f64, y: f64) -> f64 {
    return 2.0 * (x * x - x + 1.0) + 2.0 * (y * y - y + 1.0);
}

pub fn compare_offense(object_vector: &Vec<Vec<f64>>, compare_vec: &Vec<Vec<f64>>) -> f64 {
    let mut max_value = 0.0 as f64;
    let n = object_vector.len();

    for i in 0..n {
        for j in 0..n {
            max_value = max_value.max((compare_vec[i][j] - object_vector[i][j]).abs());
        }
    }

    return max_value;
}

pub fn make_vec(red_vec: &Vec<Vec<f64>>, black_vec: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = red_vec.len();
    let mut vec = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if (i + j) % 2 == 0 {
                vec[i][j] = red_vec[i][j];
            } else {
                vec[i][j] = black_vec[i][j];
            }
        }
    }

    return vec;
    
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    
    let pool = if args.len() > 1{
        let n: usize = args[1].parse().unwrap_or(1);
        ThreadPoolBuilder::new().num_threads(n)
    }
    else{
        ThreadPoolBuilder::new()
    };

    pool.build_global().unwrap();


    let start = Instant::now();

    // Threads number

    let n = 512;
    let u = (n - 1) as f64;
    let h = 1.0 / u;
    let steps = 500000;

    let mut vec = vec![vec![0.0; n]; n];
    let mut p_vec = vec![vec![0.0; n]; n];

    for i in 0..n {
        let x = i as f64 / u;
        let val = x * x - x + 1.0;
        vec[i][0] = val;
        vec[i][n - 1] = val;
        vec[0][i] = val;
        vec[n - 1][i] = val;
    }

    for i in 0..n {
        for j in 0..n {
            let x = i as f64 / u;
            let y = j as f64 / u;
            p_vec[i][j] = (x * x - x + 1.0) * (y * y - y + 1.0);
        }
    } 
    let mut red_vec = vec.clone();
    let mut black_vec = vec.clone();
    let mut vec_copy = vec.clone();
    let mut counts = 0;

    for _step in 0..steps {
        counts += 1;

        red_vec
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, row)| {
                if i == 0 || i == n - 1 {
                    return;
                }

                for j in 1..n - 1 {
                    if (i + j) % 2 == 0 {
                        let x = i as f64 / u;
                        let y = j as f64 / u;
                        row[j] = 0.25
                            * (black_vec[i - 1][j]
                                + black_vec[i + 1][j]
                                + black_vec[i][j - 1]
                                + black_vec[i][j + 1]
                                - h * h * f(x, y));
                    }
                }
            });

        //vec = vec_copy.clone();

        black_vec
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, row)| {
                if i == 0 || i == n - 1 {
                    return;
                }
                for j in 1..n - 1 {
                    if (i + j) % 2 == 1 {
                        let x = i as f64 / u;
                        let y = j as f64 / u;
                        row[j] = 0.25
                            * (red_vec[i - 1][j]
                                + red_vec[i + 1][j]
                                + red_vec[i][j - 1]
                                + red_vec[i][j + 1]
                                - h * h * f(x, y));
                    }
                }
            });

        if counts % 2000 == 0 {
            vec_copy = make_vec(&red_vec, &black_vec);
            let difference = compare_offense(&p_vec, &vec_copy);
            println!(
                "{} iteration, max difference between real and close result is: {}",
                counts, difference
            );
            if difference.abs() <= 0.001 {
                println!("gol");
                break;
            }
        }
    }

    let file = File::create("u_result.txt")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "x\ty\tu(x,y)")?;

    for i in 0..n {
        for j in 0..n {
            let x = i as f64 / u;
            let y = j as f64 / u;
            writeln!(writer, "{:.5}\t{:.5}\t{:.8}", x, y, vec_copy[i][j])?;
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed in main loop: {:?}", duration);
    println!("done");

    Ok(())
}
