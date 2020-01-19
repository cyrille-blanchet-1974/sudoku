use super::accessor::*;
use super::cell::*;
use super::column::*;
use super::constant::*;
use super::line::*;
use super::square::*;
use std::convert::TryInto;

//Grid => 81 cells
pub struct Grid {
    cells: Vec<Cell>, //the cells are stored in a Vec
    acc: Accessor,    //methods to retreive cells by coordinates
    lines: Vec<Line>,
    columns: Vec<Column>,
    squares: Vec<Square>,
    resolved: bool,
}

impl Default for Grid {
    fn default() -> Self {
        let mut cells = Vec::new();
        //construct all cells
        for i in 0..GRIDSIZE {
            cells.push(Cell::new(i.try_into().unwrap()));
        }
        let acc = Accessor::new();
        let mut lines = Vec::new();
        //construct all lines
        for _i in 0..COLUMNSIZE {
            lines.push(Line::default());
        }
        let mut columns = Vec::new();
        //construct all columns
        for _i in 0..LINESIZE {
            columns.push(Column::default());
        }
        let mut squares = Vec::new();
        let c = Cardinal::C;
        //construct all Squares
        for _i in c.get_all() {
            squares.push(Square::default());
        }
        Grid {
            cells,
            acc,
            lines,
            columns,
            squares,
            resolved: false,
        }
    }
}

impl Grid {
    pub fn set_val(&mut self, line: u8, column: u8, val: u8) {
        let pos = coord_to_pos(line, column);
        let cell: &mut Cell = &mut (self.cells[pos]);
        cell.set_val(val);
        let c: usize = (column - 1).try_into().unwrap();
        let col: &mut Column = &mut (self.columns[c]);
        col.add_a_known_value(val);
        let l: usize = (line - 1).try_into().unwrap();
        let lin: &mut Line = &mut (self.lines[l]);
        lin.add_a_known_value(val);
        let s: usize = (pos_to_square(pos).get_value() - 1).try_into().unwrap();
        let squ: &mut Square = &mut (self.squares[s]);
        squ.add_a_known_value(val);
    }

    /**
     * check if resolved
     */
    pub fn is_resolved(&mut self) -> bool {
        if !self.resolved {
            for i in 0..GRIDSIZE {
                let pos: usize = i.try_into().unwrap();
                let cell: &mut Cell = &mut (self.cells[pos]);
                if !cell.is_resolved() {
                    return false;
                }
            }
            self.resolved = true;
        }
        self.resolved
    }

    fn get_resolved(&mut self) -> Vec<u8> {
        let mut res = Vec::new();
        for i in 0..GRIDSIZE {
            let pos: usize = i.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[pos]);
            if cell.is_resolved() {
                res.push(i);
            }
        }
        res
    }

    /*
    for a square and a value, if all lines but one and all columns but one are solved then
    the cell in the remainig line and column si solved with this value

    return true if found at least a new value for a cell
    */
    fn resolve_lvl2(&mut self) -> bool {
        if self.is_resolved() {
            return false;
        }
        println!();
        print!("Lvl2->");
        let mut resolve_some = false;
        //iter on squares
        let squ = Cardinal::C;
        for sq in squ.get_all() {
            //iter on values
            for value in 1..=MAX {
                if self.resolve_lvl2_square_val(sq, value) {
                    print!(" -Found a value {} in square {:?}", value, sq);
                    resolve_some = true
                }
            }
        }
        resolve_some
    }
    /**
     * check a value in a square
     * return true if a new cell is solved
     */
    fn resolve_lvl2_square_val(&mut self, squ: Cardinal, value: u8) -> bool {
        //check if the value is already in the square
        if self.check_value_in_square(squ, value) {
            return false;
        }

        //check if all but one line solved
        let mut unsolved_line = 255;
        for l in squ.get_lines() {
            //if unsolved in this line
            if !self.check_value_in_line(l, value) {
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
            if !self.check_value_in_column(c, value) {
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
        let cell: &mut Cell = &mut (self.cells[pos]);
        if cell.is_resolved() {
            return false;
        }
        //at this point only one line and one column unsolved => it is now
        self.set_val(unsolved_line, unsolved_column, value);
        true
    }

    /**
     * check if value is solved in a square
     */
    fn check_value_in_square(&mut self, s: Cardinal, val: u8) -> bool {
        //check if value is resolve
        let pos: usize = (s.get_value() - 1).try_into().unwrap();
        let squ: &mut Square = &mut (self.squares[pos]);
        squ._is_known(val)
    }
    /**
     * check if value is solved in a line
     */
    fn check_value_in_line(&mut self, line: u8, val: u8) -> bool {
        //check if value is resolve
        let pos: usize = (line - 1).try_into().unwrap();
        let lin: &mut Line = &mut (self.lines[pos]);
        lin.is_known(val)
    }
    /**
     * check if value is solved in a column
     */
    fn check_value_in_column(&mut self, column: u8, val: u8) -> bool {
        //check if value is resolve
        let pos: usize = (column - 1).try_into().unwrap();
        let col: &mut Column = &mut (self.columns[pos]);
        col.is_known(val)
    }

    pub fn resolve(&mut self) -> bool {
        let res1 = self.resolve_lvl1();
        let res2 = self.resolve_lvl2();
        let res3 = self.resolve_lvl3();
        res1 || res2 || res3
    }

    /*
    If a cell is resolved then his value is in no other cells of the same Row,
    in no other cells of the same column and in no other cells of the same square
    return true if found one or more
    */
    fn resolve_lvl1(&mut self) -> bool {
        if self.is_resolved() {
            return false;
        }
        println!();
        print!("Lvl1->");
        //get resolved cells positions
        let mut resolved = self.get_resolved();
        let prev_count = resolved.len();
        //for each resolved cell call lvl1
        for p in resolved {
            self.resolve_lvl1_val(p);
        }
        resolved = self.get_resolved();
        //if count of solved has change then we found something
        resolved.len() != prev_count
    }

    /*
    If a cell is resolved then his value is in no other cells of the same Row,
    in no other cells of the same column and in no other cells of the same square
    */
    fn resolve_lvl1_val(&mut self, p: u8) {
        //get value of the received cell
        let pos: usize = p.try_into().unwrap();
        let cell: &mut Cell = &mut (self.cells[pos]);
        let val = match cell.get_answer() {
            None => return, //if not solve... noting to do
            Some(x) => x,
        };
        //get other cells
        let clean = self.get_to_clean(p);
        let val: usize = val.try_into().unwrap();
        //remove the value to all the others
        for c in clean {
            let cc: usize = c.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[cc]);
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
                    self.set_val(line, col, x);
                }
            }
        }
    }

    /*
     from a cell retrieve the cells of the same line, same column and same square
     but not the original one
    */
    fn get_to_clean(&self, p: u8) -> Vec<u8> {
        let mut res = Vec::new();
        let pos: usize = p.try_into().unwrap();
        let cell: &Cell = &(self.cells[pos]);
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
    If a value is not in the possible of a line less a cell then the cell has this values
    idem for column
    and square
    return true if found one or more
    */
    fn resolve_lvl3(&mut self) -> bool {
        if self.is_resolved() {
            return false;
        }
        println!();
        print!("Lvl3->");
        let mut solve_one_at_least = false;
        for v in 1..=MAX {
            let val: usize = v.try_into().unwrap();
            for line in 1..=COLUMNSIZE {
                if self.resolve_lvl3_line(line, val) {
                    solve_one_at_least = true;
                }
            }
            for column in 1..=LINESIZE {
                if self.resolve_lvl3_column(column, val) {
                    solve_one_at_least = true;
                }
            }
            let c = Cardinal::C;
            for square in c.get_all() {
                if self.resolve_lvl3_square(square, val) {
                    solve_one_at_least = true;
                }
            }
        }
        solve_one_at_least
    }

    fn resolve_lvl3_line(&mut self, line: u8, val: usize) -> bool {
        if self.check_value_in_line(line, val.try_into().unwrap()) {
            //if val already solved in the line
            return false;
        }
        let mut unsolve = 255;
        //iterate on all cells of the line
        for p in self.acc.get_line(line) {
            let pos: usize = p.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[pos]);
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
            self.set_val(coord.0, coord.1, v);
            print!(
                " -Found a value {} on cell {} (l:{}/c:{})  ",
                val, unsolve, coord.0, coord.1
            );
            return true;
        }
        false
    }

    fn resolve_lvl3_column(&mut self, column: u8, val: usize) -> bool {
        if self.check_value_in_column(column, val.try_into().unwrap()) {
            //if val already solved in the column
            return false;
        }
        let mut unsolve = 255;
        //iterate on all cells of the line
        for p in self.acc.get_column(column) {
            let pos: usize = p.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[pos]);
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
            self.set_val(coord.0, coord.1, v);
            print!(
                " -Found a value {} on cell {} (l:{}/c:{})  ",
                val, unsolve, coord.0, coord.1
            );
            return true;
        }
        false
    }

    fn resolve_lvl3_square(&mut self, square: Cardinal, val: usize) -> bool {
        if self.check_value_in_square(square, val.try_into().unwrap()) {
            //if val already solved in the square
            return false;
        }
        let mut unsolve = 255;
        //iterate on all cells of the line
        for p in self.acc.get_square(square) {
            let pos: usize = p.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[pos]);
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
            self.set_val(coord.0, coord.1, v);
            print!(
                " -Found a value {} on cell {} (l:{}/c:{})  ",
                val, unsolve, coord.0, coord.1
            );
            return true;
        }
        false
    }

    /**
     * check if resolved
     */
    pub fn display(&mut self) {
        println!();
        println!("-------------------------------");
        for line in 1..=LINESIZE {
            print!("|");
            for column in 1..=COLUMNSIZE {
                let pos = coord_to_pos(line, column);
                let cell: &Cell = &self.cells[pos];
                match cell.get_answer() {
                    None => print!(" ? "),
                    Some(x) => print!(" {} ", x),
                };
                if column % 3 == 0 {
                    print!("|");
                }
            }
            println!();
            if line % 3 == 0 {
                println!("-------------------------------");
            }
        }
        if self.is_resolved() {
            println!("Puzzle solved!");
        }
    }

    pub fn debug(&mut self) {
        println!("-------------------------------DEBUG-------------------------------");
        let mut nb = 0;
        for i in 0..GRIDSIZE {
            let pos: usize = i.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[pos]);
            if cell.debug() {
                nb += 1;
            }
            if nb == 3 {
                println!();
                nb = 0;
            }
        }
        if nb != 3 {
            println!();
        }
        println!("-------------------------------DEBUG-------------------------------");
    }

    pub fn check_puzzle(&self) -> bool {
        let attendu = 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1;
        let mut c;
        //ctl by line
        for line in 1..=LINESIZE {
            c = 0;
            for column in 1..=COLUMNSIZE {
                let pos = coord_to_pos(line, column);
                let cell: &Cell = &self.cells[pos];
                match cell.get_answer() {
                    None => c += 0,
                    Some(x) => c += x,
                };
            }
            if c != attendu {
                println!("unckeck line {} => {}", line, c);
                return false;
            }
        }
        //ctl by column
        for column in 1..=COLUMNSIZE {
            c = 0;
            for line in 1..=LINESIZE {
                let pos = coord_to_pos(line, column);
                let cell: &Cell = &self.cells[pos];
                match cell.get_answer() {
                    None => c += 0,
                    Some(x) => c += x,
                };
            }
            if c != attendu {
                println!("unckeck column {} => {}", column, c);
                return false;
            }
        }
        //ctl by square
        let card = Cardinal::C;
        for c in card.get_all() {
            if !self.check_square(c) {
                return false;
            }
        }
        true
    }
    fn check_square(&self, card: Cardinal) -> bool {
        let attendu = 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1;
        let c = card.get_coord();
        let l1 = (c.0).0;
        let l2 = (c.1).0;
        let c1 = (c.0).1;
        let c2 = (c.1).1;
        let mut c = 0;
        for column in c1..=c2 {
            for line in l1..=l2 {
                let pos = coord_to_pos(line, column);
                let cell: &Cell = &self.cells[pos];
                match cell.get_answer() {
                    None => c += 0,
                    Some(x) => c += x,
                };
            }
        }
        if c != attendu {
            println!("uncheck square {},{} {},{} => {}", c1, c2, l1, l2, c);
            return false;
        }
        true
    }
}

#[test]
fn resolution_test() {
    let mut g = Grid::default();
    assert_eq!(false, g.is_resolved());
}

#[test]
fn display_test() {
    let mut g = Grid::default();
    g.display();
    g.set_val(1, 1, 1);
    g.set_val(1, 2, 2);
    g.set_val(1, 3, 3);
    g.set_val(1, 4, 4);
    g.set_val(1, 5, 5);
    g.set_val(1, 6, 6);
    g.set_val(1, 7, 7);
    g.set_val(1, 8, 8);
    g.set_val(1, 9, 9);
    g.set_val(2, 1, 4);
    g.set_val(2, 2, 5);
    g.set_val(2, 3, 6);
    g.set_val(2, 4, 7);
    g.set_val(2, 5, 8);
    g.set_val(2, 6, 9);
    g.set_val(2, 7, 1);
    g.set_val(2, 8, 2);
    g.set_val(2, 9, 3);
    g.set_val(3, 1, 7);
    g.set_val(3, 2, 8);
    g.set_val(3, 3, 9);
    g.set_val(3, 4, 1);
    g.set_val(3, 5, 2);
    g.set_val(3, 6, 3);
    g.set_val(3, 7, 4);
    g.set_val(3, 8, 5);
    g.set_val(3, 9, 6);
    g.set_val(4, 1, 2);
    g.set_val(4, 2, 3);
    g.set_val(4, 3, 4);
    g.set_val(4, 4, 5);
    g.set_val(4, 5, 6);
    g.set_val(4, 6, 7);
    g.set_val(4, 7, 8);
    g.set_val(4, 8, 9);
    g.set_val(4, 9, 1);
    g.set_val(5, 1, 5);
    g.set_val(5, 2, 6);
    g.set_val(5, 3, 7);
    g.set_val(5, 4, 8);
    g.set_val(5, 5, 9);
    g.set_val(5, 6, 1);
    g.set_val(5, 7, 2);
    g.set_val(5, 8, 3);
    g.set_val(5, 9, 4);
    g.set_val(6, 1, 8);
    g.set_val(6, 2, 9);
    g.set_val(6, 3, 1);
    g.set_val(6, 4, 2);
    g.set_val(6, 5, 3);
    g.set_val(6, 6, 4);
    g.set_val(6, 7, 5);
    g.set_val(6, 8, 6);
    g.set_val(6, 9, 7);
    g.set_val(7, 1, 3);
    g.set_val(7, 2, 4);
    g.set_val(7, 3, 5);
    g.set_val(7, 4, 6);
    g.set_val(7, 5, 7);
    g.set_val(7, 6, 8);
    g.set_val(7, 7, 9);
    g.set_val(7, 8, 1);
    g.set_val(7, 9, 2);
    g.set_val(8, 1, 6);
    g.set_val(8, 2, 7);
    g.set_val(8, 3, 8);
    g.set_val(8, 4, 9);
    g.set_val(8, 5, 1);
    g.set_val(8, 6, 2);
    g.set_val(8, 7, 3);
    g.set_val(8, 8, 4);
    g.set_val(8, 9, 5);
    g.set_val(9, 1, 9);
    g.set_val(9, 2, 1);
    g.set_val(9, 3, 2);
    g.set_val(9, 4, 3);
    g.set_val(9, 5, 4);
    g.set_val(9, 6, 5);
    g.set_val(9, 7, 6);
    g.set_val(9, 8, 7);
    g.set_val(9, 9, 8);
    g.display();
}

#[test]
fn check_test() {
    let mut g = Grid::default();
    assert_eq!(false, g.check_puzzle());
    g.set_val(1, 1, 1);
    g.set_val(1, 2, 2);
    g.set_val(1, 3, 3);
    g.set_val(1, 4, 4);
    g.set_val(1, 5, 5);
    g.set_val(1, 6, 6);
    g.set_val(1, 7, 7);
    g.set_val(1, 8, 8);
    g.set_val(1, 9, 9);

    g.set_val(2, 1, 4);
    g.set_val(2, 2, 5);
    g.set_val(2, 3, 6);
    g.set_val(2, 4, 7);
    g.set_val(2, 5, 8);
    g.set_val(2, 6, 9);
    g.set_val(2, 7, 1);
    g.set_val(2, 8, 2);
    g.set_val(2, 9, 3);

    g.set_val(3, 1, 7);
    g.set_val(3, 2, 8);
    g.set_val(3, 3, 9);
    g.set_val(3, 4, 1);
    g.set_val(3, 5, 2);
    g.set_val(3, 6, 3);
    g.set_val(3, 7, 4);
    g.set_val(3, 8, 5);
    g.set_val(3, 9, 6);

    g.set_val(4, 1, 2);
    g.set_val(4, 2, 3);
    g.set_val(4, 3, 4);
    g.set_val(4, 4, 5);
    g.set_val(4, 5, 6);
    g.set_val(4, 6, 7);
    g.set_val(4, 7, 8);
    g.set_val(4, 8, 9);
    g.set_val(4, 9, 1);

    g.set_val(5, 1, 5);
    g.set_val(5, 2, 6);
    g.set_val(5, 3, 7);
    g.set_val(5, 4, 8);
    g.set_val(5, 5, 9);
    g.set_val(5, 6, 1);
    g.set_val(5, 7, 2);
    g.set_val(5, 8, 3);
    g.set_val(5, 9, 4);

    g.set_val(6, 1, 8);
    g.set_val(6, 2, 9);
    g.set_val(6, 3, 1);
    g.set_val(6, 4, 2);
    g.set_val(6, 5, 3);
    g.set_val(6, 6, 4);
    g.set_val(6, 7, 5);
    g.set_val(6, 8, 6);
    g.set_val(6, 9, 7);

    g.set_val(7, 1, 3);
    g.set_val(7, 2, 4);
    g.set_val(7, 3, 5);
    g.set_val(7, 4, 6);
    g.set_val(7, 5, 7);
    g.set_val(7, 6, 8);
    g.set_val(7, 7, 9);
    g.set_val(7, 8, 1);
    g.set_val(7, 9, 2);

    g.set_val(8, 1, 6);
    g.set_val(8, 2, 7);
    g.set_val(8, 3, 8);
    g.set_val(8, 4, 9);
    g.set_val(8, 5, 1);
    g.set_val(8, 6, 2);
    g.set_val(8, 7, 3);
    g.set_val(8, 8, 4);
    g.set_val(8, 9, 5);

    g.set_val(9, 1, 9);
    g.set_val(9, 2, 1);
    g.set_val(9, 3, 2);
    g.set_val(9, 4, 3);
    g.set_val(9, 5, 4);
    g.set_val(9, 6, 5);
    g.set_val(9, 7, 6);
    g.set_val(9, 8, 7);
    g.set_val(9, 9, 8);

    g.display();
    assert_eq!(true, g.check_puzzle());
}
impl Clone for Grid {
    fn clone(&self) -> Grid {
        let mut ce = Vec::new();
        for v in &self.cells {
            ce.push(v.clone());
        }
        let mut li = Vec::new();
        for v in &self.lines {
            li.push(v.clone());
        }
        let mut co = Vec::new();
        for v in &self.columns {
            co.push(v.clone());
        }
        let mut sq = Vec::new();
        for v in &self.squares {
            sq.push(v.clone());
        }
        Grid {
            cells: ce,
            acc: Accessor::new(), //Accessor always contains sames datas
            lines: li,
            columns: co,
            squares: sq,
            resolved: self.resolved,
        }
    }
}
