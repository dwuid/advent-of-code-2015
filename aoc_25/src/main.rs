
fn code_index(row: usize, column: usize) -> usize {
    let gauss = |n| (n * (n + 1)) / 2;

    /*
     * (There probably is a fancy closed-form expression for this I don't know,
     * bear with me.)
     *
     * f(1, c) = 1 + 2 + ... c               = (c * (c + 1)) / 2.
     * f(r, c) = f(1, c) + c + (c + 1) + ...
     *
     * Values downwards a column increase by c, c + 1, c + 2, ... compared to
     * the column's top value f(1, c).
     *
     * For a value f(r, c) with r > 1, the increase relative to f(r - 1, c) is
     * c + r - 2.  <=>  f(r, c) = f(r - 1, c) + c + r - 2; r > 1.
     *
     * The total increase relative to f(1, c) then is
     *    = (1 + 2 + ... + (c - 1)  + c + ... + (c + r - 2))
     *    - (1 + 2 + ... + (c - 1))
     *
     *    = gauss(c + r - 2) - gauss(c - 1).
     */

    gauss(column) + gauss(row + column - 2) - gauss(column - 1)
}

fn main() {
    let challenge  = (2981, 3075);
    let iterations = code_index(challenge.0, challenge.1) - 1;

    let base: u64 = 20151125;
    let modulus   = 33554393;

    // Poor man's powmod.
    let mut factor = 1;

    for _ in 0..iterations {
        factor = (factor * 252533) % modulus;
    }

    println!("Take this code, it's dangerous to go alone: {}.",
             (base * factor) % modulus);
}

