mod polynomial;
mod report;
mod individual;
mod ecosystem;
mod random_util;

use crate::ecosystem::Ecosystem;

fn main()
{
    //let points = vec![(0.0, 0.0), (1.0, 1.0), (4.0, 4.0)];
    //let points = vec![(0.0, 2.0), (1.0, 2.0), (5.0, 2.0)];

    //let points = vec![(0.0, 0.0), (1.0, 2.0), (50.0, 100.0), (-25.0, -50.0)];

    let points = vec![(-1.0, 1.0), (0.0, 0.0), (1.0, 1.0), (10.0, 100.0),
                      (-10.0, 100.0), (-3.0, 9.0)];

    let mut system = Ecosystem::new(9, 2, points);

    let report = system.run(30);

    println!("{}", report.to_string());
}
