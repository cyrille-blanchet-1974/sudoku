use super::accessor::*;
use super::cell::*;
use super::constant::*;
use super::grid::*;
use std::convert::TryInto;

pub struct Resolver {
    step: u32,
    nblvl1: u32,
    nblvl1ko: u32,
    nblvl2: u32,
    nblvl2ko: u32,
    nblvl3: u32,
    nblvl3ko: u32,
    nblvl4guess: u32,
    nblvl4wrongguess: u32,
    acc: Accessor, //methods to retreive cells by coordinates
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            step: 1,
            acc: Accessor::new(),
            nblvl1: 0,
            nblvl1ko: 0,
            nblvl2: 0,
            nblvl2ko: 0,
            nblvl3: 0,
            nblvl3ko: 0,
            nblvl4guess: 0,
            nblvl4wrongguess: 0,
        }
    }

    fn display_stats(&mut self) {
        println!(
            "Called {} times level 1 ({} times with no new result)",
            self.nblvl1, self.nblvl1ko
        );
        println!(
            "Called {} times level 2 ({} times with no new result)",
            self.nblvl2, self.nblvl2ko
        );
        println!(
            "Called {} times level 3 ({} times with no new result)",
            self.nblvl3, self.nblvl3ko
        );
        println!(
            "Made {} guess at level 4, {} of those were wrong",
            self.nblvl4guess, self.nblvl4wrongguess
        );
    }

    pub fn go(&mut self, g: &mut Grid, debug: bool) -> bool {
        g.display();
        //if already resolved...
        if !g.is_resolved() {
            //loop until no more to solve
            loop {
                if debug {
                    println!("--------------------step {}--------------------", self.step);
                }
                if !self.resolve(g) {
                    break;
                }
                if debug {
                    g.display();
                    if !g.is_valid() {
                        println!("Error in the grid!");
                        return false;
                    }
                }
            }
        }
        let res = g.is_resolved();
        if !res {
            return self.resolve_lvl4(g, debug);
        }
        g.display();
        if res {
            println!("Grid resolved!!!!!");
            self.display_stats();
        } else if debug {
            g.debug();
        }
        res
    }

    fn resolve_lvl4(&mut self, g: &mut Grid, debug: bool) -> bool {
        //let run level 4 -> tries
        //first made a copy of
        let mut sav = g.clone();
        let sav_step = self.step;
        self.nblvl4guess += 1;
        //second find a unsolved cell
        match g.get_first_unsolved() {
            None => {
                println!("Strange error: grid not solved by with no unsolved cell (or unsoved cell with no possibles values)");
                false
            }
            Some(val) => {
                //on the saved grid, let try this value on the cell
                println!("Lvl4-> try value {} on cell l:{}/c:{}", val.2, val.0, val.1);
                sav.set_val(val.0, val.1, val.2);
                if self.go(&mut sav, debug) {
                    //we found the solution
                    true
                } else {
                    println!(
                        "Lvl4-> wrong guess so value {} is not possible for on cell l:{}/c:{} -> restoring previous grid (from step {})",
                        val.2, val.0, val.1,sav_step
                    );
                    self.nblvl4wrongguess += 1;
                    //wrong guess -> remove this value from the possibles of the original grid and continue
                    g.remove_a_possible(val.0, val.1, val.2);
                    self.go(g, debug)
                }
            }
        }
    }

    pub fn resolve(&mut self, g: &mut Grid) -> bool {
        self.step += 1;
        //try 3 first type of resolution
        //only solution when removing founds of the same line columnand square
        let res1 = self.resolve_lvl1(g);
        self.nblvl1 += 1;
        if !res1 {
            self.nblvl1ko += 1;
        }
        //value fount in two other line and two other column of a square
        let res2 = self.resolve_lvl2(g);
        self.nblvl2 += 1;
        if !res2 {
            self.nblvl2ko += 1;
        }
        //value not in the possible of other cells of the same line OR
        //value not in the possible of other cells of the same column OR
        //value not in the possible of other cells of the same square
        let res3 = self.resolve_lvl3(g);
        self.nblvl3 += 1;
        if !res3 {
            self.nblvl3ko += 1;
        }
        res1 || res2 || res3
    }

    /*
    If a cell is resolved then his value is in no other cells of the same Row,
    in no other cells of the same column and in no other cells of the same square
    return true if found one or more
    */
    fn resolve_lvl1(&mut self, g: &mut Grid) -> bool {
        if g.is_resolved() {
            return false;
        }
        print!("Lvl1->");
        //get resolved cells positions
        let mut resolved = g.get_resolved();
        let prev_count = resolved.len();
        //for each resolved cell call lvl1
        for p in resolved {
            self.resolve_lvl1_val(g, p);
        }
        resolved = g.get_resolved();
        println!();
        //if count of solved has change then we found something
        resolved.len() != prev_count
    }

    /*
    If a cell is resolved then his value is in no other cells of the same Row,
    in no other cells of the same column and in no other cells of the same square
    */
    fn resolve_lvl1_val(&mut self, g: &mut Grid, p: u8) {
        //get value of the received cell
        let pos: usize = p.try_into().unwrap();
        let cell: &mut Cell = &mut (g.get_cell(pos));
        let val = match cell.get_answer() {
            None => return, //if not solve...nothing to do...but should not have been called
            Some(x) => x,
        };
        //get other cells
        let clean = self.resolve_lvl1_get_to_clean(g, p);
        let val: usize = val.try_into().unwrap();
        //remove the value to all the others
        for c in clean {
            let cc: usize = c.try_into().unwrap();
            let cell: &mut Cell = &mut (g.get_cell(cc));
            if cell.is_resolved() {
                continue;
            }
            if cell.remove_a_possible_and_verify(val) {
                //removing a possible we found the answer of the cell
                //so we must clean lines,columns and squares
                if let Some(x) = cell.get_answer() {
                    let col = cell.get_column();
                    let line = cell.get_line();
                    print!(
                        " -Found a value {} on cell {} (l:{}/c:{})  ",
                        x, cc, line, col
                    );
                    g.set_val(line, col, x);
                }
            }
        }
    }

    /*
    from a cell retrieve the cells of the same line, same column and same square
    but not the original one
    */
    fn resolve_lvl1_get_to_clean(&self, g: &mut Grid, p: u8) -> Vec<u8> {
        let mut res = Vec::new();
        let pos: usize = p.try_into().unwrap();
        let cell: &Cell = &(g.get_cell(pos));
        let lin = self.acc.get_line(cell.get_line());
        for l in lin {
            if l != p {
                res.push(l);
            }
        }
        let col = self.acc.get_column(cell.get_column());
        for c in col {
            if c != p {
                res.push(c);
            }
        }
        let squ = self.acc.get_square(cell.get_square());
        for s in squ {
            if s != p {
                res.push(s);
            }
        }
        res
    }

    /*
    for a square and a value, if all lines but one and all columns but one are solved then
    the cell in the remainig line and column si solved with this value

    return true if found at least a new value for a cell
    */
    fn resolve_lvl2(&mut self, g: &mut Grid) -> bool {
        if g.is_resolved() {
            return false;
        }
        print!("Lvl2->");
        let mut resolve_some = false;
        //iter on squares
        let squ = Cardinal::C;
        for sq in squ.get_all() {
            //iter on values
            for value in 1..=MAX {
                if self.resolve_lvl2_square_val(g, sq, value) {
                    resolve_some = true
                }
            }
        }
        println!();
        resolve_some
    }
    /**
     * check a value in a square
     * return true if a new cell is solved
     */
    fn resolve_lvl2_square_val(&mut self, g: &mut Grid, squ: Cardinal, value: u8) -> bool {
        //check if the value is already in the square
        if g.check_value_in_square(squ, value) {
            return false;
        }

        //check if all but one line solved
        let mut unsolved_line = 255;
        for l in squ.get_lines() {
            //if unsolved in this line
            if !g.check_value_in_line(l, value) {
                //first unsolved ?
                if unsolved_line == 255 {
                    unsolved_line = l;
                } else {
                    //if two lines unsolved then let go
                    return false;
                }
            }
        }
        if unsolved_line == 255 {
            //if tree line solved then let go
            return false;
        }
        //now check the columns
        //check if all but one column solved
        let mut unsolved_column = 255;
        for c in squ.get_columns() {
            //if unsolved in this column
            if !g.check_value_in_column(c, value) {
                //first unsolved ?
                if unsolved_column == 255 {
                    unsolved_column = c;
                } else {
                    //if two columns unsolved then let go
                    return false;
                }
            }
        }
        if unsolved_column == 255 {
            //if tree columns solved then let go
            return false;
        }
        //check if cell is already solved
        let pos: usize = coord_to_pos(unsolved_line, unsolved_column)
            .try_into()
            .unwrap();
        let cell: &mut Cell = &mut (g.get_cell(pos));
        if cell.is_resolved() {
            return false;
        }
        //at this point only one line and one column unsolved => it is now
        g.set_val(unsolved_line, unsolved_column, value);
        print!(
            " -Found a value {} in cell {} of square {:?} (l:{}/c:{})",
            value, pos, squ, unsolved_line, unsolved_column
        );
        true
    }

    /*
    If a value is not in the possible of a line less a cell then the cell has this values
    idem for column
    and square
    return true if found one or more
    */
    fn resolve_lvl3(&mut self, g: &mut Grid) -> bool {
        if g.is_resolved() {
            return false;
        }
        print!("Lvl3->");
        let mut solve_one_at_least = false;
        for v in 1..=MAX {
            let val: usize = v.try_into().unwrap();
            for line in 1..=COLUMNSIZE {
                if self.resolve_lvl3_line(g, line, val) {
                    solve_one_at_least = true;
                }
            }
            for column in 1..=LINESIZE {
                if self.resolve_lvl3_column(g, column, val) {
                    solve_one_at_least = true;
                }
            }
            let c = Cardinal::C;
            for square in c.get_all() {
                if self.resolve_lvl3_square(g, square, val) {
                    solve_one_at_least = true;
                }
            }
        }
        println!();
        solve_one_at_least
    }

    fn resolve_lvl3_line(&mut self, g: &mut Grid, line: u8, val: usize) -> bool {
        if g.check_value_in_line(line, val.try_into().unwrap()) {
            //if val already solved in the line
            return false;
        }
        let mut unsolve = 255;
        //iterate on all cells of the line
        for p in self.acc.get_line(line) {
            let pos: usize = p.try_into().unwrap();
            let cell: &mut Cell = &mut (g.get_cell(pos));
            if cell.is_a_possible(val) {
                if unsolve != 255 {
                    //second possible? -> 2 possibles -> not a solution
                    return false;
                }
                unsolve = p;
            }
        }
        if unsolve != 255 {
            //found
            let pos: usize = unsolve.try_into().unwrap();
            let v: u8 = val.try_into().unwrap();
            let coord = pos_to_coord(pos);
            g.set_val(coord.0, coord.1, v);
            print!(
                " -Found a value {} on cell {} (l:{}/c:{})  ",
                val, unsolve, coord.0, coord.1
            );
            return true;
        }
        false
    }

    fn resolve_lvl3_column(&mut self, g: &mut Grid, column: u8, val: usize) -> bool {
        if g.check_value_in_column(column, val.try_into().unwrap()) {
            //if val already solved in the column
            return false;
        }
        let mut unsolve = 255;
        //iterate on all cells of the line
        for p in self.acc.get_column(column) {
            let pos: usize = p.try_into().unwrap();
            let cell: &mut Cell = &mut (g.get_cell(pos));
            if cell.is_a_possible(val) {
                if unsolve != 255 {
                    //second possible? -> 2 possibles -> not a solution
                    return false;
                }
                unsolve = p;
            }
        }
        if unsolve != 255 {
            //found
            let pos: usize = unsolve.try_into().unwrap();
            let v: u8 = val.try_into().unwrap();
            let coord = pos_to_coord(pos);
            g.set_val(coord.0, coord.1, v);
            print!(
                " -Found a value {} on cell {} (l:{}/c:{})  ",
                val, unsolve, coord.0, coord.1
            );
            return true;
        }
        false
    }

    fn resolve_lvl3_square(&mut self, g: &mut Grid, square: Cardinal, val: usize) -> bool {
        if g.check_value_in_square(square, val.try_into().unwrap()) {
            //if val already solved in the square
            return false;
        }
        let mut unsolve = 255;
        //iterate on all cells of the line
        for p in self.acc.get_square(square) {
            let pos: usize = p.try_into().unwrap();
            let cell: &mut Cell = &mut (g.get_cell(pos));
            if cell.is_a_possible(val) {
                if unsolve != 255 {
                    //second possible? -> 2 possibles -> not a solution
                    return false;
                }
                unsolve = p;
            }
        }
        if unsolve != 255 {
            //found
            let pos: usize = unsolve.try_into().unwrap();
            let v: u8 = val.try_into().unwrap();
            let coord = pos_to_coord(pos);
            g.set_val(coord.0, coord.1, v);
            print!(
                " -Found a value {} on cell {} (l:{}/c:{})  ",
                val, unsolve, coord.0, coord.1
            );
            return true;
        }
        false
    }
}
