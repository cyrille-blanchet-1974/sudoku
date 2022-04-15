use super::super::objects::accessor::*;
use super::super::objects::cell::*;
use super::super::objects::grid::*;
use std::convert::TryInto;

pub struct ResolverLvl1 {
    acc: Accessor,
}

impl ResolverLvl1 {
    pub fn new(side: u8) -> ResolverLvl1 {
        ResolverLvl1 {
            acc: Accessor::new(side),
        }
    }

    /*
    If a cell is resolved then his value is in no other cells of the same Row,
    in no other cells of the same column and in no other cells of the same square
    return true if found one or more
    */
    pub fn resolve(&self, g: &mut Grid) -> bool {
        if g.resolved() {
            return false;
        }
        g.clear_trace();
        //get resolved cells positions
        let mut resolved = g.get_resolved();
        let prev_count = resolved.len();
        //for each resolved cell call lvl1
        for p in resolved {
            self.resolve_val(g, p);
        }
        resolved = g.get_resolved();
        //if count of solved has change then we found something
        resolved.len() != prev_count
    }

    /*
    If a cell is resolved then his value is in no other cells of the same Row,
    in no other cells of the same column and in no other cells of the same square
    */
    fn resolve_val(&self, g: &mut Grid, p: u16) {
        //get value of the received cell
        let pos: usize = p.try_into().unwrap();
        let cell: &mut Cell = g.get_cell(pos);
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
            let cell: &mut Cell = g.get_cell(cc);
            if cell.is_resolved() {
                continue;
            }
            if cell.remove_candidate_and_verify(val) {
                //removing a possible we found the answer of the cell
                //so we must clean lines,columns and squares
                if let Some(x) = cell.get_answer() {
                    let col = cell.get_column();
                    let line = cell.get_line();
                    let trc = format!(" {}/{}={}", line, col, x);
                    g.add_trace(trc);
                    g.set_val(line, col, x, CellType::Found);
                }
            }
        }
    }

    /*
    from a cell retrieve the cells of the same line, same column and same square
    but not the original one
    */
    fn get_to_clean(&self, g: &mut Grid, p: u16) -> Vec<u16> {
        let mut res = Vec::new();
        let pos: usize = p.try_into().unwrap();
        let cell: &Cell = g.get_cell(pos);
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
