use super::accessor::*;
use super::cell::*;
use super::constant::*;
use super::grid::*;

pub struct ResolverLvl2 {
    debug : bool,
    trace : String,
}

impl ResolverLvl2 {
    pub fn new(debug : bool) -> ResolverLvl2 {
        ResolverLvl2 {
            debug,
            trace : String::new(),
        }
    }

    /*
    for a square and a value, if all lines but one and all columns but one are solved then
    the cell in the remainig line and column si solved with this value

    return true if found at least a new value for a cell
    */
    pub fn resolve(&mut self, g: &mut Grid) -> bool {
        if g.is_resolved() {
            return false;
        }
        self.trace = "".to_string();
        let mut resolve_some = false;
        //iter on squares
        let squ = Cardinal::C;
        for sq in squ.get_all() {
            //iter on values
            for value in 1..=MAX {
                if self.resolve_square_val(g, sq, value) {
                    resolve_some = true
                }
            }
        }
        if self.debug && self.trace != "" {
            println!("{}",self.trace);
        }
        resolve_some
    }
    /**
     * check a value in a square
     * return true if a new cell is solved
     */
    fn resolve_square_val(&mut self, g: &mut Grid, squ: Cardinal, value: u8) -> bool {
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
        let pos: usize = coord_to_pos(unsolved_line, unsolved_column);
        let cell: &mut Cell = &mut (g.get_cell(pos));
        if cell.is_resolved() {
            return false;
        }
        //at this point only one line and one column unsolved => it is now
        g.set_val(unsolved_line, unsolved_column, value, CellType::FOUND);
        if self.trace == "" {
            self.trace = "Lvl2->".to_string();
        }
        let trc = format!(" -Found a value {} in cell {} of square {:?} (l:{}/c:{})", value, pos, squ, unsolved_line, unsolved_column);
        self.trace.push_str(&trc);
        true
    }
}
