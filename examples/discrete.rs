extern crate seal;

use seal::pair::{
    strategy::discrete::{global::Strategy as GlobalStrategy, local::Strategy as LocalStrategy},
    Alignment, Alignments, Step, Strategy as StrategyTrait,
};

fn trace(_x_seq: &Vec<isize>, _y_seq: &Vec<isize>, alignment: &Alignment<isize>) {
    let mut x_vec: Vec<char> = vec![];
    let mut y_vec: Vec<char> = vec![];
    for step in alignment.steps() {
        match step {
            Step::Align { x: _x, y: _y } => {
                print!("=");
                x_vec.push('=');
                y_vec.push('=');
            }
            Step::Delete { x: _x } => {
                print!(">");
                x_vec.push('=');
                y_vec.push('-');
            }
            Step::Insert { y: _y } => {
                print!("<");
                x_vec.push('-');
                y_vec.push('=');
            }
        }
    }

    print!("\n");

    let x_str: String = x_vec.into_iter().collect();
    let y_str: String = y_vec.into_iter().collect();

    for (x, y) in x_str.chars().zip(y_str.chars()) {
        if x == y {
            print!("=");
        } else {
            print!("|");
        }
    }
    print!("\n");

    println!("{}", x_str);
    println!("{}", y_str);
}

fn align<T>(label: &str, seq_x: &[isize], seq_y: &[isize], strategy: T)
where
    T: StrategyTrait<isize, Score = isize>,
{
    let sequence_x: Vec<_> = seq_x.to_owned();
    let sequence_y: Vec<_> = seq_y.to_owned();
    let alignment_set: Alignments<isize> =
        strategy.alignments(&sequence_x[..], &sequence_y[..], |x, y| {
            // if x == y {
            //     -1
            // } else {
            //     1
            // }
            (x - y).abs() - 1
        });

    println!("{:?}", alignment_set.matrix());
    if let Some(alignment) = alignment_set.alignment() {
        println!("{}:", label);
        println!("{:#?}", alignment);
        trace(&sequence_x, &sequence_y, &alignment);
    } else {
        println!("No alignment found.");
    }
}

fn main() {
    let seq_a = vec![0, 1, 5, 10, 10, 10, 5, 1, 0];
    let seq_b = vec![0, 10, 10, 10, 0];

    let global = GlobalStrategy::default();

    align("Global Alignment", &seq_a[..], &seq_b[..], global);

    println!("");

    let local = LocalStrategy::default();

    align("Local Alignment", &seq_a[..], &seq_b[..], local);

    println!("");
}
