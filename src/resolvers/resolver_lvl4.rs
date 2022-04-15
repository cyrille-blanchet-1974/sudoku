use super::super::objects::accessor::*;
use super::super::objects::cell::*;
use super::super::objects::grid::*;
use std::convert::TryInto;

pub struct ResolverLvl4 {
    trace: String,
}

impl ResolverLvl4 {
    pub fn new() -> ResolverLvl4 {
        ResolverLvl4 {
            trace: String::new(),
        }
    }

    /*
        get a string containg what was found
    */
    pub fn get_trace(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.trace);
        output
    }

    /*
    X-wing resolve
    return true if found one or more
    */
    pub fn resolve(&mut self, g: &mut Grid) -> bool {
        if g.resolved() {
            return false;
        }
        self.trace = "".to_string();
        let mut solve_one_at_least = false;
        if self.resolve_line(g) {
            solve_one_at_least = true;
        }
        if self.resolve_column(g) {
            solve_one_at_least = true;
        }

        solve_one_at_least
    }

    fn resolve_line(&mut self, g: &mut Grid) -> bool {
        let mut trouve = false;
        let acc = Accessor::new(g.get_metrics().get_square_side());
        let max = g.get_metrics().get_max();
        let nb_line = g.get_metrics().get_nb_line();
        let nb_column = g.get_metrics().get_nb_column();
        //loop values
        for v in 1..=max {
            let val: usize = v.try_into().unwrap();
            let mut tab = Vec::new();
            //loop lines
            'lines: for line in 1..=nb_line {
                let mut t = (0, 0); // count   and positions
                                    //count is nb of cell with val in possibles
                                    //positions is 'binairy' positions of column with val possible
                let mut p: u32 = 100_000_000;
                for column in 1..=nb_column {
                    let pos = acc.coordconverter.coord_to_pos(line, column);
                    let cell: &mut Cell = g.get_cell(pos);
                    if cell.is_resolved() {
                        match cell.get_answer() {
                            None => {}
                            Some(x) => {
                                if x == v {
                                    tab.push((0, 0));
                                    //ignore this column
                                    continue 'lines;
                                }
                            }
                        };
                    }
                    if cell.candidate(val) {
                        t.0 += 1;
                        t.1 += p;
                    }
                    //col 1  => 100 000 000
                    //col 2  =>  10 000 000
                    //...
                    //col 8  =>        10
                    //col 9  =>         1
                    p /= 10;
                }
                tab.push(t);
                //check columns with value resolved if only 2 columns keep
                //if 2 lines (and only two) with same two columns remove val off possible for other columns of the two lines
            }
            //check if we find 2 lines with only 2 count and the same position
            //loop on tab
            for line in 1..=nb_line {
                let l: usize = line.try_into().unwrap();
                let t = tab[l - 1];
                //find a cell with count = 2
                if t.0 == 2 {
                    //then search another one
                    for line2 in line + 1..=nb_line {
                        let l2: usize = line2.try_into().unwrap();
                        let t2 = tab[l2 - 1];
                        //find a cell with count = 2
                        if t2.0 == 2 && t.1 == t2.1 {
                            //found 2 lines each of them has value v possible in the sames columns
                            let c = self.decode(t2.1, 9);
                            //Set type of cells to 'Xwing"
                            self.set_xwing(
                                g,
                                line,
                                line2,
                                c.0.try_into().unwrap(),
                                c.1.try_into().unwrap(),
                            );
                            let trc = format!(
                                "xwing found for val {} in lines {} and {}  and columns {} and {}",
                                val, l, l2, c.0, c.1
                            );
                            self.trace.push_str(&trc);
                            //So we can remove value v of all other cells of this two columns
                            for l in 1..=nb_line {
                                if l == line {
                                    continue;
                                }
                                if l == line2 {
                                    continue;
                                }
                                let pos =
                                    acc.coordconverter.coord_to_pos(l, c.0.try_into().unwrap());
                                let cell: &mut Cell = g.get_cell(pos);
                                if !cell.is_resolved() && cell.remove_candidate_and_verify(val) {
                                    trouve = true;
                                }
                                let pos =
                                    acc.coordconverter.coord_to_pos(l, c.1.try_into().unwrap());
                                let cell: &mut Cell = g.get_cell(pos);
                                if !cell.is_resolved() && cell.remove_candidate_and_verify(val) {
                                    trouve = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        trouve
    }
    fn resolve_column(&mut self, g: &mut Grid) -> bool {
        let mut trouve = false;
        let acc = Accessor::new(g.get_metrics().get_square_side());
        let max = g.get_metrics().get_max();
        let nb_line = g.get_metrics().get_nb_line();
        let nb_column = g.get_metrics().get_nb_column();
        //loop values
        for v in 1..=max {
            let val: usize = v.try_into().unwrap();
            let mut tab = Vec::new();
            //loop columns
            'cols: for column in 1..=nb_column {
                let mut t = (0, 0); // count   and positions
                                    //count is nb of cell with val in possibles
                                    //positions is 'binairy' positions of line with val possible
                let mut p: u32 = 100_000_000;
                for line in 1..=nb_line {
                    let pos = acc.coordconverter.coord_to_pos(line, column);
                    let cell: &mut Cell = g.get_cell(pos);
                    if cell.is_resolved() {
                        match cell.get_answer() {
                            None => {}
                            Some(x) => {
                                if x == v {
                                    tab.push((0, 0));
                                    //ignore this column
                                    continue 'cols;
                                }
                            }
                        };
                    }
                    if cell.candidate(val) {
                        t.0 += 1;
                        t.1 += p;
                    }
                    //line 1  => 100 000 000
                    //line 2  =>  10 000 000
                    //...
                    //line 8  =>        10
                    //line 9  =>         1
                    p /= 10;
                }
                tab.push(t);
                //check lines with value resolved if only 2 lines keep
                //if 2 columns (and only two) with same two lines remove val off possible for other lines of the two cols
            }
            //check if we find 2 cols with only 2 count and the same position
            //loop on tab
            for column in 1..=nb_column {
                let c: usize = column.try_into().unwrap();
                let t = tab[c - 1];
                //find a cell with count = 2
                if t.0 == 2 {
                    //then search another one
                    for column2 in column + 1..=nb_column {
                        let c2: usize = column2.try_into().unwrap();
                        let t2 = tab[c2 - 1];
                        //find a cell with count = 2
                        if t2.0 == 2 && t.1 == t2.1 {
                            //found 2 cols each of them has value v possible in the sames lines
                            let l = self.decode(t2.1, 9);
                            //Set type of cells to 'Xwing"
                            self.set_xwing(
                                g,
                                l.0.try_into().unwrap(),
                                l.1.try_into().unwrap(),
                                column,
                                column2,
                            );
                            let trc = format!(
                                "xwing found for val {} in columns {} and {}  and lines {} and {}",
                                val, c, c2, l.0, l.1
                            );
                            self.trace.push_str(&trc);
                            //So we can remove value v of all other cells of this two lines
                            for c in 1..=nb_column {
                                if c == column {
                                    continue;
                                }
                                if c == column2 {
                                    continue;
                                }
                                let pos =
                                    acc.coordconverter.coord_to_pos(l.0.try_into().unwrap(), c);
                                let cell: &mut Cell = g.get_cell(pos);
                                if !cell.is_resolved() && cell.remove_candidate_and_verify(val) {
                                    trouve = true;
                                }
                                let pos =
                                    acc.coordconverter.coord_to_pos(l.1.try_into().unwrap(), c);
                                let cell: &mut Cell = g.get_cell(pos);
                                if !cell.is_resolved() && cell.remove_candidate_and_verify(val) {
                                    trouve = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        trouve
    }

    fn set_xwing(&mut self, g: &mut Grid, l1: u8, l2: u8, c1: u8, c2: u8) {
        let acc = Accessor::new(g.get_metrics().get_square_side());
        let pos = acc.coordconverter.coord_to_pos(l1, c1);
        let cell: &mut Cell = g.get_cell(pos);
        cell.set_type(CellType::Xwing);
        let pos = acc.coordconverter.coord_to_pos(l1, c2);
        let cell: &mut Cell = g.get_cell(pos);
        cell.set_type(CellType::Xwing);
        let pos = acc.coordconverter.coord_to_pos(l2, c1);
        let cell: &mut Cell = g.get_cell(pos);
        cell.set_type(CellType::Xwing);
        let pos = acc.coordconverter.coord_to_pos(l2, c2);
        let cell: &mut Cell = g.get_cell(pos);
        cell.set_type(CellType::Xwing);
    }

    fn decode(&mut self, d: u32, linesize: u8) -> (usize, usize) {
        let mut p: u32 = 100_000_000;
        let mut u1 = 0;
        let mut u2 = 0;
        let mut d32 = d;
        for line in 1..=linesize {
            if d32 >= p {
                d32 -= p;
                if u1 == 0 {
                    u1 = line.try_into().unwrap();
                } else {
                    u2 = line.try_into().unwrap();
                    return (u1, u2);
                }
            }
            p /= 10;
        }
        (u1, u2)
    }
}

#[test]
fn test_decode() {
    let mut l4 = ResolverLvl4::new();
    assert_eq!((1, 2), l4.decode(110000000, 9));
    assert_eq!((1, 9), l4.decode(100000001, 9));
    assert_eq!((3, 6), l4.decode(001006000, 9));
}

#[test]
fn test_resolve() {
    let mut g = Grid::new(3);
    g.set_val(1, 1, 6, CellType::Origin);
    g.set_val(1, 2, 7, CellType::Origin);
    g.set_val(1, 3, 9, CellType::Origin);
    g.set_val(1, 4, 4, CellType::Origin);
    g.set_val(1, 5, 1, CellType::Origin);
    g.set_val(1, 6, 8, CellType::Origin);
    g.set_val(1, 7, 3, CellType::Origin);
    g.set_val(1, 8, 5, CellType::Origin);
    g.set_val(1, 9, 2, CellType::Origin);
    g.set_val(2, 1, 2, CellType::Origin);
    g.set_val(2, 2, 4, CellType::Origin);
    g.set_val(2, 3, 4, CellType::Origin);
    g.set_val(2, 4, 3, CellType::Origin);
    g.set_val(2, 5, 9, CellType::Origin);
    g.set_val(2, 6, 5, CellType::Origin);
    g.set_val(2, 7, 7, CellType::Origin);
    g.set_val(2, 8, 6, CellType::Origin);
    g.set_val(2, 9, 1, CellType::Origin);
    g.set_val(3, 1, 1, CellType::Origin);
    g.set_val(3, 2, 5, CellType::Origin);
    g.set_val(3, 3, 3, CellType::Origin);
    g.set_val(3, 4, 7, CellType::Origin);
    g.set_val(3, 5, 6, CellType::Origin);
    g.set_val(3, 6, 2, CellType::Origin);
    g.set_val(3, 7, 9, CellType::Origin);
    g.set_val(3, 8, 8, CellType::Origin);
    g.set_val(3, 9, 4, CellType::Origin);
    //ori.set_val(4, 1, 1, CellType::Origin);
    //ori.set_val(4, 2, 1, CellType::Origin);
    //ori.set_val(4, 3, 1, CellType::Origin);
    //ori.set_val(4, 4, 1, CellType::Origin);
    //ori.set_val(4, 5, 1, CellType::Origin);
    //ori.set_val(4, 6, 1, CellType::Origin);
    g.set_val(4, 7, 4, CellType::Origin);
    g.set_val(4, 8, 2, CellType::Origin);
    //ori.set_val(4, 9, 1, CellType::Origin);
    g.set_val(5, 1, 7, CellType::Origin);
    //ori.set_val(5, 2, 1, CellType::Origin);
    g.set_val(5, 3, 4, CellType::Origin);
    //ori.set_val(5, 4, 1, CellType::Origin);
    g.set_val(5, 5, 8, CellType::Origin);
    g.set_val(5, 6, 1, CellType::Origin);
    g.set_val(5, 7, 5, CellType::Origin);
    g.set_val(5, 8, 3, CellType::Origin);
    g.set_val(5, 9, 6, CellType::Origin);
    //ori.set_val(6, 1, 1, CellType::Origin);
    //ori.set_val(6, 2, 1, CellType::Origin);
    //ori.set_val(6, 3, 1, CellType::Origin);
    //ori.set_val(6, 4, 1, CellType::Origin);
    //ori.set_val(6, 5, 1, CellType::Origin);
    //ori.set_val(6, 6, 1, CellType::Origin);
    g.set_val(6, 7, 1, CellType::Origin);
    g.set_val(6, 8, 7, CellType::Origin);
    //ori.set_val(6, 9, 1, CellType::Origin);
    g.set_val(7, 1, 5, CellType::Origin);
    g.set_val(7, 2, 8, CellType::Origin);
    g.set_val(7, 3, 7, CellType::Origin);
    g.set_val(7, 4, 1, CellType::Origin);
    g.set_val(7, 5, 2, CellType::Origin);
    g.set_val(7, 6, 9, CellType::Origin);
    g.set_val(7, 7, 6, CellType::Origin);
    g.set_val(7, 8, 4, CellType::Origin);
    g.set_val(7, 9, 3, CellType::Origin);
    g.set_val(8, 1, 4, CellType::Origin);
    g.set_val(8, 2, 6, CellType::Origin);
    g.set_val(8, 3, 1, CellType::Origin);
    g.set_val(8, 4, 8, CellType::Origin);
    g.set_val(8, 5, 3, CellType::Origin);
    g.set_val(8, 6, 7, CellType::Origin);
    g.set_val(8, 7, 2, CellType::Origin);
    g.set_val(8, 8, 9, CellType::Origin);
    g.set_val(8, 9, 5, CellType::Origin);
    g.set_val(9, 1, 3, CellType::Origin);
    g.set_val(9, 2, 9, CellType::Origin);
    g.set_val(9, 3, 2, CellType::Origin);
    g.set_val(9, 4, 6, CellType::Origin);
    g.set_val(9, 5, 5, CellType::Origin);
    g.set_val(9, 6, 4, CellType::Origin);
    g.set_val(9, 7, 8, CellType::Origin);
    g.set_val(9, 8, 1, CellType::Origin);
    g.set_val(9, 9, 7, CellType::Origin);

    let mut l4 = ResolverLvl4::new();
    assert_eq!(false, l4.resolve(&mut g));
}
