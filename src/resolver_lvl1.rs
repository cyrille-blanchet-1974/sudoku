use super::accessor::*;
use super::cell::*;
use super::grid::*;
use std::convert::TryInto;

pub struct ResolverLvl1 {
    acc: Accessor, //methods to retreive cells by coordinates
    debug : bool,
    trace : String,
}

impl ResolverLvl1 {
    pub fn new(debug : bool) -> ResolverLvl1 {
        ResolverLvl1 {
            acc: Accessor::new(),
            debug,
            trace : String::new(),
        }
    }

    /*
    If a cell is resolved then his value is in no other cells of the same Row,
    in no other cells of the same column and in no other cells of the same square
    return true if found one or more
    */
    pub fn resolve(&mut self, g: &mut Grid) -> bool {
        if g.is_resolved() {
            return false;
        }
        self.trace = "".to_string();
        //get resolved cells positions
        let mut resolved = g.get_resolved();
        let prev_count = resolved.len();
        //for each resolved cell call lvl1
        for p in resolved {
            self.resolve_val(g, p);
        }
        resolved = g.get_resolved();
        if self.debug && self.trace != "" {
            println!("{}",self.trace);
        }
        //if count of solved has change then we found something
        resolved.len() != prev_count
    }

    /*
    If a cell is resolved then his value is in no other cells of the same Row,
    in no other cells of the same column and in no other cells of the same square
    */
    fn resolve_val(&mut self, g: &mut Grid, p: u8) {
        //get value of the received cell
        let pos: usize = p.try_into().unwrap();
        let cell: &mut Cell = &mut (g.get_cell(pos));
        let val = match cell.get_answer() {
            None => return, //if not solve...nothing to do...but should not have been called
            Some(x) => x,
        };
        //get other cells
        let clean = self.get_to_clean(g, p);
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
                    if self.trace == "" {
                        self.trace = "Lvl1->".to_string();
                    }
                    let trc = format!(" -Found a value {} on cell {} (l:{}/c:{})  ", x, cc, line, col);
                    self.trace.push_str(&trc);
                    g.set_val(line, col, x, CellType::FOUND);
                }
            }
        }
    }

    /*
    from a cell retrieve the cells of the same line, same column and same square
    but not the original one
    */
    fn get_to_clean(&self, g: &mut Grid, p: u8) -> Vec<u8> {
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
}
