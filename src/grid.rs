use super::accessor::*;
use super::cell::*;
use super::column::*;
use super::constant::*;
use super::line::*;
use super::square::*;
use std::convert::TryInto;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

//Grid => 81 cells
pub struct Grid {
    cells: Vec<Cell>, //the cells are stored in a Vec
    acc: Accessor,    //methods to retreive cells by coordinates
    lines: Vec<Line>,
    columns: Vec<Column>,
    squares: Vec<Square>,
    resolved: bool,
    debug : bool,
}

impl Default for Grid {
    /**
     * create a default Sudoku Grid
     */
    fn default() -> Self {
        let mut cells = Vec::new();
        //construct all cells
        for i in 0..GRIDSIZE {
            cells.push(Cell::new(i.try_into().unwrap(),false));
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
            debug:false,
        }
    }
}

//methods to update Grid struct
impl Grid {
    pub fn set_debug(&mut self,debug :bool){
        self.debug=debug;
    }
    /**
     * put a value in a cell -> add it in the known values of the line/column/square of the cell
     **/
    pub fn set_val(&mut self, line: u8, column: u8, val: u8, t: CellType) {
        let pos = coord_to_pos(line, column);
        let cell: &mut Cell = &mut (self.cells[pos]);
        cell.set_val(val, t);
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
     * remove a possible value from a cell
     **/
    pub fn remove_a_possible(&mut self, line: u8, column: u8, val: u8) {
        let pos = coord_to_pos(line, column);
        let cell: &mut Cell = &mut (self.cells[pos]);
        let v: usize = val.try_into().unwrap();
        let ok = cell.remove_a_possible_and_verify(v);
        if ok {
            self.set_val(line, column, val, CellType::FOUND)
        }
    }

    pub fn get_cell(&mut self, pos: usize) -> &mut Cell {
        &mut self.cells[pos]
    }

    /**
     * check if the Grid is resolved
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

    /**
     * Verify the grid
     * return false if two cell of the same line, column or square have the same value
     * */
    pub fn is_valid(&self) -> bool {
        let mut mess;
        for line in 1..=COLUMNSIZE {
            mess = format!("line {}", line);
            let res = self.is_valid_set(self.acc.get_line(line), mess);
            if !res {
                return false;
            }
        }
        for column in 1..=LINESIZE {
            mess = format!("column {}", column);
            let res = self.is_valid_set(self.acc.get_column(column), mess);
            if !res {
                return false;
            }
        }
        let c = Cardinal::C;
        for square in c.get_all() {
            mess = format!("square {:?}", square);
            let res = self.is_valid_set(self.acc.get_square(square), mess);
            if !res {
                return false;
            }
        }
        true
    }
}

//resolve helping methods
impl Grid {
    /**
     * get the list of the resolved cells of the grid
     *
     */
    pub fn get_resolved(&mut self) -> Vec<u8> {
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

    /**
     * get first unsolved cell (in fact fist with the less possible values)
     * return a tuple containing (line,column,value) in an Option
     */
    pub fn get_first_unsolved(&mut self) -> Option<(u8, u8, u8)> {
        let mut potential = (0, 0, 0, 999); //line/column/value/nb possibles
                                            //find a cell not resolved
        for pos in 0..GRIDSIZE {
            let p: usize = pos.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[p]);
            if !cell.is_resolved() {
                //and return first of its possibles
                let poss = cell.get_possibles();
                match poss.get(0) {
                    None => continue, //seems strange that an unsolved cell has no possibles values...
                    Some(x) => {
                        //if new has less possible than previous we prefer this
                        if poss.len() < potential.3 {
                            potential = (cell.get_line(), cell.get_column(), *x, poss.len())
                        }
                    }
                };
            }
        }
        //if found ar least one
        if potential.3 != 999 {
            Some((potential.0, potential.1, potential.2))
        } else {
            None
        }
    }

    /**
     * check if value is solved in a square
     */
    pub fn check_value_in_square(&mut self, s: Cardinal, val: u8) -> bool {
        //check if value is resolve
        let pos: usize = (s.get_value() - 1).try_into().unwrap();
        let squ: &mut Square = &mut (self.squares[pos]);
        squ.is_known(val)
    }
    /**
     * check if value is solved in a line
     */
    pub fn check_value_in_line(&mut self, line: u8, val: u8) -> bool {
        //check if value is resolve
        let pos: usize = (line - 1).try_into().unwrap();
        let lin: &mut Line = &mut (self.lines[pos]);
        lin.is_known(val)
    }

    /**
     * check if value is solved in a column
     */
    pub fn check_value_in_column(&mut self, column: u8, val: u8) -> bool {
        //check if value is resolve
        let pos: usize = (column - 1).try_into().unwrap();
        let col: &mut Column = &mut (self.columns[pos]);
        col.is_known(val)
    }

    /**
     * Check in a set of cells if a value is present more than one time
     * */
     pub fn is_valid_set(&self, set: Vec<u8>, text: String) -> bool {
        let mut count = Vec::new();
        for _i in 0..MAX {
            count.push(0);
        }
        for v in set {
            let pos: usize = v.try_into().unwrap();
            let cell: &Cell = &self.cells[pos];
            match cell.get_answer() {
                None => {}
                Some(a) => {
                    let pos: usize = (a - 1).try_into().unwrap();
                    count[pos] += 1;
                }
            };
        }
        for i in 0..MAX {
            let pos: usize = i.try_into().unwrap();
            match count.get(pos) {
                None => {}
                Some(val) => {
                    if *val > 1 {
                        if self.debug {
                            println!("Value {} found more than once in {}!", i + 1, text);
                        }
                        return false;
                    }
                }
            };
        }
        true
    }
}

//display and debug methods
impl Grid {
    /**
     * display the actual grid
     */
    pub fn display(&mut self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout
            .set_color(
                ColorSpec::new()
                    .set_bg(Some(Color::Black))
                    .set_fg(Some(Color::White)),
            )
            .unwrap();
        writeln!(&mut stdout, "╔═════════╦═════════╦═════════╗").unwrap();
        for line in 1..=LINESIZE {
            write!(&mut stdout, "║").unwrap();
            for column in 1..=COLUMNSIZE {
                let pos = coord_to_pos(line, column);
                let cell: &Cell = &self.cells[pos];
                match cell.get_answer() {
                    None => {
                        stdout
                            .set_color(
                                ColorSpec::new()
                                    .set_bg(Some(Color::Black))
                                    .set_fg(Some(Color::White)),
                            )
                            .unwrap();
                        write!(&mut stdout, " ? ").unwrap();
                    }
                    Some(x) => {
                        match cell.get_type() {
                            CellType::FOUND => stdout
                                .set_color(
                                    ColorSpec::new()
                                        .set_bg(Some(Color::Green))
                                        .set_fg(Some(Color::White)),
                                )
                                .unwrap(),
                            CellType::GUESS => stdout
                                .set_color(
                                    ColorSpec::new()
                                        .set_bg(Some(Color::Red))
                                        .set_fg(Some(Color::White)),
                                )
                                .unwrap(),
                            CellType::ORIGIN => stdout
                                .set_color(
                                    ColorSpec::new()
                                        .set_bg(Some(Color::Blue))
                                        .set_fg(Some(Color::White)),
                                )
                                .unwrap(),
                            CellType::UNKNOWN => stdout
                                .set_color(
                                    ColorSpec::new()
                                        .set_bg(Some(Color::Black))
                                        .set_fg(Some(Color::White)),
                                )
                                .unwrap(),
                        }
                        write!(&mut stdout, " {} ", x).unwrap();
                    }
                };
                stdout
                    .set_color(
                        ColorSpec::new()
                            .set_bg(Some(Color::Black))
                            .set_fg(Some(Color::White)),
                    )
                    .unwrap();
                if column % 3 == 0 {
                    write!(&mut stdout, "║").unwrap();
                }
            }
            println!();

            if line % 9 == 0 {
                writeln!(&mut stdout, "╚═════════╩═════════╩═════════╝").unwrap();
            } else if line % 3 == 0 {
                writeln!(&mut stdout, "╟═════════╬═════════╬═════════╢").unwrap();
            }
        }
        if self.is_resolved() {
            println!("Puzzle solved!");
        }
    }

    /**
     * display debug info  (ex remaining possibles of the cells)
     **/
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
}

impl Grid {
    pub fn copy_from(&mut self,g :Grid) {
        self.cells.clear();
        for v in g.cells {
            self.cells.push(v.clone());
        }
        self.lines.clear();
        for v in g.lines {
            self.lines.push(v.clone());
        }
        self.columns.clear();
        for v in g.columns {
            self.columns.push(v.clone());
        }
        self.squares.clear();
        for v in g.squares {
            self.squares.push(v.clone());
        }
    }
}

impl Clone for Grid {
    fn clone(&self) -> Grid {
        let mut cells = Vec::new();
        for v in &self.cells {
            cells.push(v.clone());
        }
        let mut lines = Vec::new();
        for v in &self.lines {
            lines.push(v.clone());
        }
        let mut columns = Vec::new();
        for v in &self.columns {
            columns.push(v.clone());
        }
        let mut squares = Vec::new();
        for v in &self.squares {
            squares.push(v.clone());
        }
        Grid {
            cells,
            acc: Accessor::new(), //Accessor always contains sames datas
            lines,
            columns,
            squares,
            resolved: self.resolved,
            debug:self.debug,
        }        
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
    g.set_val(1, 1, 1, CellType::ORIGIN);
    g.set_val(1, 2, 2, CellType::ORIGIN);
    g.set_val(1, 3, 3, CellType::ORIGIN);
    g.set_val(1, 4, 4, CellType::ORIGIN);
    g.set_val(1, 5, 5, CellType::ORIGIN);
    g.set_val(1, 6, 6, CellType::ORIGIN);
    g.set_val(1, 7, 7, CellType::ORIGIN);
    g.set_val(1, 8, 8, CellType::ORIGIN);
    g.set_val(1, 9, 9, CellType::ORIGIN);
    g.set_val(2, 1, 4, CellType::ORIGIN);
    g.set_val(2, 2, 5, CellType::ORIGIN);
    g.set_val(2, 3, 6, CellType::ORIGIN);
    g.set_val(2, 4, 7, CellType::ORIGIN);
    g.set_val(2, 5, 8, CellType::ORIGIN);
    g.set_val(2, 6, 9, CellType::ORIGIN);
    g.set_val(2, 7, 1, CellType::ORIGIN);
    g.set_val(2, 8, 2, CellType::ORIGIN);
    g.set_val(2, 9, 3, CellType::ORIGIN);
    g.set_val(3, 1, 7, CellType::ORIGIN);
    g.set_val(3, 2, 8, CellType::ORIGIN);
    g.set_val(3, 3, 9, CellType::ORIGIN);
    g.set_val(3, 4, 1, CellType::ORIGIN);
    g.set_val(3, 5, 2, CellType::ORIGIN);
    g.set_val(3, 6, 3, CellType::ORIGIN);
    g.set_val(3, 7, 4, CellType::ORIGIN);
    g.set_val(3, 8, 5, CellType::ORIGIN);
    g.set_val(3, 9, 6, CellType::ORIGIN);
    g.set_val(4, 1, 2, CellType::ORIGIN);
    g.set_val(4, 2, 3, CellType::ORIGIN);
    g.set_val(4, 3, 4, CellType::ORIGIN);
    g.set_val(4, 4, 5, CellType::ORIGIN);
    g.set_val(4, 5, 6, CellType::ORIGIN);
    g.set_val(4, 6, 7, CellType::ORIGIN);
    g.set_val(4, 7, 8, CellType::ORIGIN);
    g.set_val(4, 8, 9, CellType::ORIGIN);
    g.set_val(4, 9, 1, CellType::ORIGIN);
    g.set_val(5, 1, 5, CellType::ORIGIN);
    g.set_val(5, 2, 6, CellType::ORIGIN);
    g.set_val(5, 3, 7, CellType::ORIGIN);
    g.set_val(5, 4, 8, CellType::ORIGIN);
    g.set_val(5, 5, 9, CellType::ORIGIN);
    g.set_val(5, 6, 1, CellType::ORIGIN);
    g.set_val(5, 7, 2, CellType::ORIGIN);
    g.set_val(5, 8, 3, CellType::ORIGIN);
    g.set_val(5, 9, 4, CellType::ORIGIN);
    g.set_val(6, 1, 8, CellType::ORIGIN);
    g.set_val(6, 2, 9, CellType::ORIGIN);
    g.set_val(6, 3, 1, CellType::ORIGIN);
    g.set_val(6, 4, 2, CellType::ORIGIN);
    g.set_val(6, 5, 3, CellType::ORIGIN);
    g.set_val(6, 6, 4, CellType::ORIGIN);
    g.set_val(6, 7, 5, CellType::ORIGIN);
    g.set_val(6, 8, 6, CellType::ORIGIN);
    g.set_val(6, 9, 7, CellType::ORIGIN);
    g.set_val(7, 1, 3, CellType::ORIGIN);
    g.set_val(7, 2, 4, CellType::ORIGIN);
    g.set_val(7, 3, 5, CellType::ORIGIN);
    g.set_val(7, 4, 6, CellType::ORIGIN);
    g.set_val(7, 5, 7, CellType::ORIGIN);
    g.set_val(7, 6, 8, CellType::ORIGIN);
    g.set_val(7, 7, 9, CellType::ORIGIN);
    g.set_val(7, 8, 1, CellType::ORIGIN);
    g.set_val(7, 9, 2, CellType::ORIGIN);
    g.set_val(8, 1, 6, CellType::ORIGIN);
    g.set_val(8, 2, 7, CellType::ORIGIN);
    g.set_val(8, 3, 8, CellType::ORIGIN);
    g.set_val(8, 4, 9, CellType::ORIGIN);
    g.set_val(8, 5, 1, CellType::ORIGIN);
    g.set_val(8, 6, 2, CellType::ORIGIN);
    g.set_val(8, 7, 3, CellType::ORIGIN);
    g.set_val(8, 8, 4, CellType::ORIGIN);
    g.set_val(8, 9, 5, CellType::ORIGIN);
    g.set_val(9, 1, 9, CellType::ORIGIN);
    g.set_val(9, 2, 1, CellType::ORIGIN);
    g.set_val(9, 3, 2, CellType::ORIGIN);
    g.set_val(9, 4, 3, CellType::ORIGIN);
    g.set_val(9, 5, 4, CellType::ORIGIN);
    g.set_val(9, 6, 5, CellType::ORIGIN);
    g.set_val(9, 7, 6, CellType::ORIGIN);
    g.set_val(9, 8, 7, CellType::ORIGIN);
    g.set_val(9, 9, 8, CellType::ORIGIN);
    g.display();
}

#[test]
fn check_is_valid() {
    let mut g = Grid::default();
    assert_eq!(true, g.is_valid());
    g.set_val(1, 1, 1, CellType::ORIGIN);
    assert_eq!(true, g.is_valid());
    g.set_val(1, 3, 3, CellType::ORIGIN);
    assert_eq!(true, g.is_valid());
    g.set_val(1, 4, 4, CellType::ORIGIN);
    assert_eq!(true, g.is_valid());
    g.set_val(1, 5, 5, CellType::ORIGIN);
    assert_eq!(true, g.is_valid());
    g.set_val(1, 6, 6, CellType::ORIGIN);
    assert_eq!(true, g.is_valid());
    g.set_val(1, 7, 7, CellType::ORIGIN);
    assert_eq!(true, g.is_valid());
    g.set_val(1, 8, 8, CellType::ORIGIN);
    assert_eq!(true, g.is_valid());
    g.set_val(1, 9, 9, CellType::ORIGIN);
    assert_eq!(true, g.is_valid());

    let mut g2 = g.clone();
    g2.set_val(2, 1, 1, CellType::ORIGIN); //two 1 on samae column
    assert_eq!(false, g2.is_valid());

    let mut g2 = g.clone();
    g2.set_val(1, 2, 1, CellType::ORIGIN); //Two 1 on same line
    assert_eq!(false, g2.is_valid());

    let mut g2 = g.clone();
    g2.set_val(3, 3, 1, CellType::ORIGIN); //Two 1 on same square
    assert_eq!(false, g2.is_valid());
}

#[test]
fn clone_grid_test() {
    let mut ori = Grid::default();
    ori.set_val(1, 1, 1, CellType::ORIGIN);
    ori.set_val(2, 4, 1, CellType::ORIGIN);
    ori.set_val(3, 7, 1, CellType::ORIGIN);
    ori.set_val(4, 2, 1, CellType::ORIGIN);
    ori.set_val(5, 5, 1, CellType::ORIGIN);
    ori.set_val(6, 8, 1, CellType::ORIGIN);
    ori.set_val(7, 3, 1, CellType::ORIGIN);
    let mut copy = ori.clone();
    assert_eq!(ori.get_resolved(), copy.get_resolved());
    ori.set_val(8, 6, 1, CellType::ORIGIN);
    assert_ne!(ori.get_resolved(), copy.get_resolved());
}
