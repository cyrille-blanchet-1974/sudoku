use super::super::objects::cell::*;
use super::super::objects::grid::*;

pub struct ResolverLvl5 {}

impl ResolverLvl5 {
    pub fn new() -> ResolverLvl5 {
        ResolverLvl5 {}
    }

    /*
    double same pair removes
    return true if found one or more
    */
    pub fn resolve(&self, g: &mut Grid) -> bool {
        if g.resolved() {
            return false;
        }
        g.clear_trace();
        let mut solve_one_at_least = false;
        if self.resolve_line(g) {
            solve_one_at_least = true;
        }
        if self.resolve_column(g) {
            solve_one_at_least = true;
        }

        solve_one_at_least
    }

    fn resolve_line(&self, g: &mut Grid) -> bool {
        let mut trouve = false;
        let nb_line = g.get_metrics().get_nb_line();
        for line in 1..=nb_line {
            let res = g.check_pairs_line(line);
            if res == None {
                continue;
            }
            let res = res.unwrap();
            {
                for c in res.2 {
                    let cell: &mut Cell = g.get_cell(c.into());
                    if cell.is_resolved() {
                        continue;
                    }
                    if cell.remove_candidate_and_verify(res.0.into())
                        || cell.remove_candidate_and_verify(res.1.into())
                    {
                        //removing a possible we found the answer of the cell
                        //so we must clean lines,columns and squares
                        if let Some(x) = cell.get_answer() {
                            let col = cell.get_column();
                            let line = cell.get_line();
                            let trc = format!(" {}/{}={}", line, col, x);
                            g.add_trace(trc);
                            g.set_val(line, col, x, CellType::Found);
                            trouve = true;
                        }
                    }
                }
            }
        }
        trouve
    }
    fn resolve_column(&self, g: &mut Grid) -> bool {
        let mut trouve = false;
        let nb_column = g.get_metrics().get_nb_column();
        for column in 1..=nb_column {
            let res = g.check_pairs_column(column);
            if res == None {
                continue;
            }
            let res = res.unwrap();
            {
                for c in res.2 {
                    let cell: &mut Cell = g.get_cell(c.into());
                    if cell.is_resolved() {
                        continue;
                    }
                    if cell.remove_candidate_and_verify(res.0.into())
                        || cell.remove_candidate_and_verify(res.1.into())
                    {
                        //removing a possible we found the answer of the cell
                        //so we must clean lines,columns and squares
                        if let Some(x) = cell.get_answer() {
                            let col = cell.get_column();
                            let line = cell.get_line();
                            let trc = format!(" {}/{}={}", line, col, x);
                            g.add_trace(trc);
                            g.set_val(line, col, x, CellType::Found);
                            trouve = true;
                        }
                    }
                }
            }
        }
        trouve
    }
}
