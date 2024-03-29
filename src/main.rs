mod grid_filler;
mod objects;
mod read;
mod resolvers;
mod ui;

use grid_filler::*;
use objects::grid::*;
use resolvers::resolver::*;
use resolvers::resolver_force::ResolverForce;
use resolvers::unicity::VerifyUnicity;
use std::time::SystemTime;
use ui::*;

pub fn clever_solving(g: &mut Grid, debug: bool, display: bool) -> bool {
    let mut r = Resolver::new(debug, display);
    println!("****Initial data for the grid****");
    g.display();
    let start_elapse = SystemTime::now();
    let res = r.go(g);
    let end = SystemTime::now();
    let tps = end
        .duration_since(start_elapse)
        .expect("ERROR computing duration!");
    println!("Duration={:?}", tps);

    println!("****Final data for the grid****");
    g.display();
    g.legend();
    let state = if g.resolved() {
        "Resolved"
    } else {
        "NOT Resolved"
    };
    println!("Grid resolution finished!!!!! => {}", state);

    r.display_stats();
    res
}

pub fn raw_solving(g: &mut Grid, debug: bool) -> bool {
    println!("****Initial data for the grid****");
    g.display();
    let start_elapse = SystemTime::now();
    let mut force = ResolverForce::new(debug, g);
    let res = force.resolve();
    let end = SystemTime::now();
    let tps = end
        .duration_since(start_elapse)
        .expect("ERROR computing duration!");
    println!("Duration={:?}", tps);
    res
}

pub fn verify_unicity(g: &mut Grid) -> bool {
    println!("Check if multiple solutions");
    let start_elapse = SystemTime::now();
    let mut v = VerifyUnicity::new(g.clone());
    let res = v.unique();
    let end = SystemTime::now();
    let tps = end
        .duration_since(start_elapse)
        .expect("ERROR computing duration!");
    println!("Duration={:?}", tps);
    if !res {
        println!("Grid has more than one solution -> not a valid Sudoku");
    } else {
        println!("We found a unique solution");
    }
    res
}

fn main() {
    println!("Sudoku resolution!");
    let mut debug = false;
    let mut display = false;
    let mut g = sample(debug);
    println!(
        "size = {}x{}",
        g.get_metrics().get_nb_line(),
        g.get_metrics().get_nb_column()
    );
    loop {
        println!("1:change grid");
        println!("2:clever solving");
        println!("3:raw solving");
        println!("4:count solutions");
        println!("97:toggle debugging (actual:{})", debug);
        println!("98:toggle display (actual:{})", display);
        println!("99:quit");
        match read_u16("Your choice?".to_string()) {
            None => {
                continue;
            }
            Some(97) => {
                debug = !debug;
            }
            Some(98) => {
                display = !display;
            }
            Some(1) => {
                if let Some(x) = choose_grid(debug) {
                    g = x
                }
            }
            Some(2) => {
                clever_solving(&mut g, debug, display);
            }
            Some(3) => {
                raw_solving(&mut g, debug);
            }
            Some(4) => {
                verify_unicity(&mut g);
            }
            Some(99) => {
                println!("Sudoku resolution End!");
                return;
            }
            _ => {
                continue;
            }
        }
    }
}

#[test]
fn clever_solve_test() {
    let mut g = sample(false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = easy(false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = medium(false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = difficult(false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = diabolical(false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = highest(false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = mindless(false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = hardest(false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = from_disk("test/easy.txt".to_string(), false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = from_disk("test/medium.txt".to_string(), false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = from_disk("test/difficult.txt".to_string(), false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = from_disk("test/diabolic.txt".to_string(), false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = from_disk("test/pascal.txt".to_string(), false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = from_disk("test/pascal2.txt".to_string(), false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = from_disk("test/pascal3.txt".to_string(), false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = from_disk("test/m.txt".to_string(), false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = from_disk("test/hardest.txt".to_string(), false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = sample22(false);
    assert_eq!(true, clever_solving(&mut g, false, false));
    let mut g = sample44(false);
    assert_eq!(true, clever_solving(&mut g, false, false));
}

#[test]
fn raw_solve_test() {
    let mut g = sample(false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = easy(false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = medium(false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = difficult(false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = diabolical(false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = highest(false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = mindless(false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = hardest(false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = from_disk("test/easy.txt".to_string(), false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = from_disk("test/medium.txt".to_string(), false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = from_disk("test/difficult.txt".to_string(), false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = from_disk("test/diabolic.txt".to_string(), false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = from_disk("test/pascal.txt".to_string(), false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = from_disk("test/pascal2.txt".to_string(), false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = from_disk("test/pascal3.txt".to_string(), false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = from_disk("test/m.txt".to_string(), false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = from_disk("test/hardest.txt".to_string(), false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = sample22(false);
    assert_eq!(true, raw_solving(&mut g, false));
    let mut g = sample44(false);
    assert_eq!(true, raw_solving(&mut g, false));
}

#[test]
fn uniq_test() {
    let mut g = sample(false);
    assert_eq!(false, verify_unicity(&mut g));
    let mut g = easy(false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = medium(false);
    assert_eq!(false, verify_unicity(&mut g));
    let mut g = difficult(false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = diabolical(false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = highest(false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = mindless(false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = hardest(false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = from_disk("test/easy.txt".to_string(), false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = from_disk("test/medium.txt".to_string(), false);
    assert_eq!(false, verify_unicity(&mut g));
    let mut g = from_disk("test/difficult.txt".to_string(), false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = from_disk("test/diabolic.txt".to_string(), false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = from_disk("test/pascal.txt".to_string(), false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = from_disk("test/pascal2.txt".to_string(), false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = from_disk("test/pascal3.txt".to_string(), false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = from_disk("test/m.txt".to_string(), false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = from_disk("test/hardest.txt".to_string(), false);
    assert_eq!(true, verify_unicity(&mut g));
    let mut g = sample22(false);
    assert_eq!(false, verify_unicity(&mut g));
    let mut g = sample44(false);
    assert_eq!(true, verify_unicity(&mut g));
}
