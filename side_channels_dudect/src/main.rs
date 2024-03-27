use dudect_bencher::rand::Rng;
use dudect_bencher::{ctbench_main, BenchRng, Class, CtRunner};

fn rand_u8_200(rng: &mut BenchRng) -> [u8; 200] {
    let mut arr = [0; 200];
    rng.fill(arr.as_mut_slice());
    arr
}

fn rand_u8_16(rng: &mut BenchRng) -> [u8; 16] {
    let mut arr = [0; 16];
    rng.fill(arr.as_mut_slice());
    arr
}

fn rand_vec(len: usize, rng: &mut BenchRng) -> Vec<u8> {
    let mut arr = vec![0u8; len];
    rng.fill(arr.as_mut_slice());
    arr
}

fn eq_vartime_yes(runner: &mut CtRunner, rng: &mut BenchRng) {
    let mut inputs: Vec<([u8; 16], [u8; 16])> = Vec::new();
    let mut classes = Vec::new();

    // Make 100,000 random pairs of vectors
    for _ in 0..100_000 {
        // Flip a coin. If true, make a pair of vectors that are equal to each
        // other and put it in the Left distribution
        if rng.gen::<bool>() {
            let v1: [u8; 16] = rand_u8_16(rng);
            let v2: [u8; 16] = v1.clone();
            inputs.push((v1, v2));
            classes.push(Class::Left);
        }
        // Otherwise, make a pair of vectors that differ at the 6th element and
        // put it in the right distribution
        else {
            let v1: [u8; 16] = rand_u8_16(rng);
            let mut v2: [u8; 16] = v1.clone();
            v2[10] = 7;
            inputs.push((v1, v2));
            classes.push(Class::Right);
        }
    }

    for (class, (u, v)) in classes.into_iter().zip(inputs.into_iter()) {
        let cmp = side_channels::Secret128bit { secret: u };

        runner.run_one(class, || cmp.check_eq_vartime(&v));
    }
}

// Crate the main function to include the bench for vec_eq
ctbench_main!(eq_vartime_yes);
