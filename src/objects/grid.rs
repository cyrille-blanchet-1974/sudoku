use super::accessor::*;
use super::cardinal::*;
use super::cell::CellType;
use super::cell::*;
use super::column::*;
use super::line::*;
use super::metrics::*;
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
    debug: bool,
    metrics: Metrics,
    trace: String,
}

//methods to update Grid struct
impl Grid {
    /**
     * create a default Sudoku Grid
     */
    pub fn new(side: u16) -> Grid {
        let mut cells = Vec::new();
        let metrics = Metrics::new(side);
        //construct all cells
        for i in 0..metrics.get_grid_size() {
            cells.push(Cell::new(i.try_into().unwrap(), false, side));
        }
        let acc = Accessor::new(side);
        let mut lines = Vec::new();
        //construct all lines
        for _i in 0..metrics.get_nb_column() {
            lines.push(Line::new(metrics.get_max()));
        }
        let mut columns = Vec::new();
        //construct all columns
        for _i in 0..metrics.get_nb_line() {
            columns.push(Column::new(metrics.get_max()));
        }
        let mut squares = Vec::new();
        let c = Cardinal::C;
        //construct all Squares
        for _i in c.get_all() {
            squares.push(Square::new(metrics.get_max()));
        }
        Grid {
            cells,
            acc,
            lines,
            columns,
            squares,
            resolved: false,
            debug: false,
            metrics,
            trace: String::new(),
        }
    }

    pub fn get_metrics(&self) -> Metrics {
        self.metrics
    }

    /*
        get a string containg what was found
    */
    pub fn get_trace(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.trace);
        output
    }
    pub fn clear_trace(&mut self) {
        self.trace.clear();
    }
    pub fn add_trace(&mut self, s: String) {
        self.trace.push_str(&s);
    }

    /**
     * how many cells not found for a given value
     */
    fn get_lefts(&mut self) -> Vec<u16> {
        //number of cell lefts for a value
        let mut lefts = Vec::new();
        for _i in 0..self.metrics.get_max() {
            lefts.push(self.metrics.get_max());
        }
        for i in 0..self.metrics.get_grid_size() {
            let pos: usize = i.try_into().unwrap();
            let cell: &mut Cell = self.get_cell(pos);
            match cell.get_answer() {
                None => {}
                Some(x) => {
                    let idx: usize = (x - 1).try_into().unwrap();
                    lefts[idx] -= 1;
                }
            }
        }
        lefts
    }

    /**
     * for a list of possible values in parameter
     * return the one with the more positions founds (or less positions not found)
     */
    pub fn less_used(&mut self, possibles: Vec<u16>) -> u16 {
        //val/nb
        let mut res = (0, 99);
        let lefts = self.get_lefts();
        for p in possibles {
            let idx: usize = (p - 1).try_into().unwrap();
            if lefts[idx] < res.1 {
                res = (p, lefts[idx])
            }
        }
        res.0
    }

    /**
     * return trus if we solve the cell or removed at least a possible valur since last call
     */
    pub fn something_has_some_change(&mut self) -> bool {
        let mut res = false;
        for i in 0..self.metrics.get_grid_size() {
            let pos: usize = i.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[pos]);
            if cell.something_has_some_change() {
                res = true;
            }
            //continue to all cells to reset bools
        }
        res
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
        for i in 0..self.metrics.get_grid_size() {
            let pos: usize = i.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[pos]);
            cell.set_debug(debug);
        }
    }
    /**
     * put a value in a cell -> add it in the known values of the line/column/square of the cell
     **/
    pub fn set_val(&mut self, line: u16, column: u16, val: u16, t: CellType) {
        let pos = self.acc.coordconverter.coord_to_pos(line, column);
        let cell: &mut Cell = &mut (self.cells[pos]);
        cell.set_val(val, t);
        let c: usize = (column - 1).try_into().unwrap();
        let col: &mut Column = &mut (self.columns[c]);
        col.add_a_known_value(val);
        let l: usize = (line - 1).try_into().unwrap();
        let lin: &mut Line = &mut (self.lines[l]);
        lin.add_a_known_value(val);
        let s: usize = (self.acc.coordconverter.pos_to_square(pos).get_value() - 1)
            .try_into()
            .unwrap();
        let squ: &mut Square = &mut (self.squares[s]);
        squ.add_a_known_value(val);
    }

    /**
     * remove a possible value from a cell
     **/
    pub fn remove_candidate(&mut self, line: u16, column: u16, val: u16) {
        let pos = self.acc.coordconverter.coord_to_pos(line, column);
        let cell: &mut Cell = &mut (self.cells[pos]);
        let v: usize = val.try_into().unwrap();
        if self.debug {
            println!("removing value {} from cell: l:{}/c:{}", val, line, column);
        }
        cell.remove_candidate_and_verify(v);
        if cell.is_resolved() {
            if self.debug {
                println!(
                    "Cell  l:{}/c:{} resolved (only one value left)",
                    line, column
                );
            }
            match cell.get_answer() {
                None => {}
                Some(x) => {
                    self.set_val(line, column, x, CellType::Found);
                }
            }
        }
    }

    pub fn get_cell(&mut self, pos: usize) -> &mut Cell {
        &mut self.cells[pos]
    }

    /**
     * check if the Grid is resolved
     */
    pub fn resolved(&mut self) -> bool {
        if !self.resolved {
            for i in 0..self.metrics.get_grid_size() {
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
    pub fn valid(&self) -> bool {
        let mut mess;
        for line in 1..=self.metrics.get_nb_column() {
            mess = format!("line {}", line);
            let res = self.is_valid_set(self.acc.get_line(line), mess);
            if !res {
                return false;
            }
        }
        for column in 1..=self.metrics.get_nb_line() {
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

#[test]
fn check_is_valid() {
    let mut g = Grid::new(3);
    assert_eq!(true, g.valid());
    g.set_val(1, 1, 1, CellType::Origin);
    assert_eq!(true, g.valid());
    g.set_val(1, 3, 3, CellType::Origin);
    assert_eq!(true, g.valid());
    g.set_val(1, 4, 4, CellType::Origin);
    assert_eq!(true, g.valid());
    g.set_val(1, 5, 5, CellType::Origin);
    assert_eq!(true, g.valid());
    g.set_val(1, 6, 6, CellType::Origin);
    assert_eq!(true, g.valid());
    g.set_val(1, 7, 7, CellType::Origin);
    assert_eq!(true, g.valid());
    g.set_val(1, 8, 8, CellType::Origin);
    assert_eq!(true, g.valid());
    g.set_val(1, 9, 9, CellType::Origin);
    assert_eq!(true, g.valid());

    let mut g2 = g.clone();
    g2.set_val(2, 1, 1, CellType::Origin); //two 1 on samae column
    assert_eq!(false, g2.valid());

    let mut g2 = g.clone();
    g2.set_val(1, 2, 1, CellType::Origin); //Two 1 on same line
    assert_eq!(false, g2.valid());

    let mut g2 = g.clone();
    g2.set_val(3, 3, 1, CellType::Origin); //Two 1 on same square
    assert_eq!(false, g2.valid());
}

//resolve helping methods
impl Grid {
    /**
     * get the list of the resolved cells of the grid
     *
     */
    pub fn get_resolved(&mut self) -> Vec<u16> {
        let mut res = Vec::new();
        for i in 0..self.metrics.get_grid_size() {
            let pos: usize = i.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[pos]);
            if cell.is_resolved() {
                res.push(i);
            }
        }
        res
    }

    /**
     * get the list of the cells unknowns
     *
     */
    pub fn get_unresolved(&mut self) -> Vec<u16> {
        let mut res = Vec::new();
        for i in 0..self.metrics.get_grid_size() {
            let pos: usize = i.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[pos]);
            if !cell.is_resolved() {
                res.push(i);
            }
        }
        res
    }

    /**
     * get a cell candidate to guessing
     * if posible a cell part of a xwing else on with the possibles
     * return cell position in an Option
     */
    pub fn get_a_guess(&mut self) -> Option<usize> {
        match self.get_first_xwing() {
            Some(x) => Some(x),
            None => self.get_first_unsolved(), /*match self.get_first_unsolved() {
                                                   Some(x) => Some(x),
                                                   None => None,
                                               },*/
        }
    }
    /**
     * get first xwing cell if exists
     * return cell position in an Option
     */
    pub fn get_first_xwing(&mut self) -> Option<usize> {
        //if a xwing exist return if
        for pos in 0..self.metrics.get_grid_size() {
            let p: usize = pos.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[p]);
            if !cell.is_resolved() && cell.get_type() == CellType::Xwing {
                return Some(p);
            }
        }
        None
    }

    /**
     * get first unsolved cell (in fact first with the less possible values)
     * return cell position in an Option
     */
    pub fn get_first_unsolved(&mut self) -> Option<usize> {
        let mut potential = (0, 999); //position/nb possibles
                                      //find a cell not resolved
        for pos in 0..self.metrics.get_grid_size() {
            let p: usize = pos.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[p]);
            if !cell.is_resolved() {
                //and return first of its possibles
                let poss = cell.get_possibles();
                if poss.len() < potential.1 {
                    potential = (p, poss.len())
                }
            }
        }
        //if found ar least one
        if potential.1 != 999 {
            Some(potential.0)
        } else {
            None
        }
    }

    /**
     * check if value is solved in a square
     */
    pub fn check_value_in_square(&mut self, s: Cardinal, val: u16) -> bool {
        //check if value is resolve
        let pos: usize = (s.get_value() - 1).try_into().unwrap();
        let squ: &mut Square = &mut (self.squares[pos]);
        squ.is_known(val)
    }
    /**
     * check if value is solved in a line
     */
    pub fn check_value_in_line(&mut self, line: u16, val: u16) -> bool {
        //check if value is resolve
        let pos: usize = (line - 1).try_into().unwrap();
        let lin: &mut Line = &mut (self.lines[pos]);
        lin.is_known(val)
    }

    /**
     * check if value is solved in a column
     */
    pub fn check_value_in_column(&mut self, column: u16, val: u16) -> bool {
        //check if value is resolve
        let pos: usize = (column - 1).try_into().unwrap();
        let col: &mut Column = &mut (self.columns[pos]);
        col.is_known(val)
    }

    /**
     * Check in a set of cells if a value is present more than one time
     * */
    pub fn is_valid_set(&self, set: Vec<u16>, text: String) -> bool {
        let mut count = Vec::new();
        for _i in 0..self.metrics.get_max() {
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
        for i in 0..self.metrics.get_max() {
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

#[test]
fn resolution_test() {
    let mut g = Grid::new(3);
    assert_eq!(false, g.resolved());
}

//display and debug methods
impl Grid {
    fn colorwrite(&self, ct: CellType, st: String) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        //set color for cell type
        match ct {
            CellType::Found => stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::Green))
                        .set_fg(Some(Color::White)),
                )
                .unwrap(),
            CellType::Guess => stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::Red))
                        .set_fg(Some(Color::White)),
                )
                .unwrap(),
            CellType::Origin => stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::Blue))
                        .set_fg(Some(Color::White)),
                )
                .unwrap(),
            CellType::Unknown => stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::Black))
                        .set_fg(Some(Color::White)),
                )
                .unwrap(),
            CellType::Xwing => stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::White))
                        .set_fg(Some(Color::Black)),
                )
                .unwrap(),
        }
        //write
        write!(&mut stdout, " {} ", st).unwrap();
        //put back colors to black and white
        stdout
            .set_color(
                ColorSpec::new()
                    .set_bg(Some(Color::Black))
                    .set_fg(Some(Color::White)),
            )
            .unwrap();
    }
    pub fn legend(&self) {
        print!("Legend: ");
        self.colorwrite(CellType::Found, "Found".to_string());
        print!(" ");
        self.colorwrite(CellType::Guess, "Guess".to_string());
        print!(" ");
        self.colorwrite(CellType::Origin, "Origin".to_string());
        print!(" ");
        self.colorwrite(CellType::Unknown, "Unknown".to_string());
        print!(" ");
        self.colorwrite(CellType::Xwing, "Xwing".to_string());
        println!();
    }

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

        let first = self.get_square_deco("╔", "═══", "╦", "╗");
        let last = self.get_square_deco("╚", "═══", "╩", "╝");
        let mid = self.get_square_deco("╟", "═══", "╬", "╢");

        writeln!(&mut stdout, "{}", first).unwrap();
        for line in 1..=self.metrics.get_nb_line() {
            write!(&mut stdout, "║").unwrap();
            for column in 1..=self.metrics.get_nb_column() {
                let pos = self.acc.coordconverter.coord_to_pos(line, column);
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
                        self.colorwrite(cell.get_type(), x.to_string());
                    }
                };
                stdout
                    .set_color(
                        ColorSpec::new()
                            .set_bg(Some(Color::Black))
                            .set_fg(Some(Color::White)),
                    )
                    .unwrap();
                if column % self.metrics.get_square_side() == 0 {
                    write!(&mut stdout, "║").unwrap();
                }
            }
            println!();

            if line % self.metrics.get_nb_column() == 0 {
                writeln!(&mut stdout, "{}", last).unwrap();
            } else if line % self.metrics.get_square_side() == 0 {
                writeln!(&mut stdout, "{}", mid).unwrap();
            }
        }
    }

    /**
     * display the actual grid en Black and white
     */
    pub fn display_bw(&mut self) {
        let linesep = self.get_square_deco("+", "---", "+", "+");
        println!("{}", linesep);
        for line in 1..=self.metrics.get_nb_line() {
            print!("|");
            for column in 1..=self.metrics.get_nb_column() {
                let pos = self.acc.coordconverter.coord_to_pos(line, column);
                let cell: &Cell = &self.cells[pos];
                match cell.get_answer() {
                    None => {
                        print!(" ? ");
                    }
                    Some(x) => {
                        print!(" {} ", x);
                    }
                };
                if column % self.metrics.get_square_side() == 0 {
                    print!("|");
                }
            }
            println!();

            if line % self.metrics.get_square_side() == 0 {
                println!("{}", linesep);
            }
        }
    }

    pub fn display_lefts(&mut self) {
        let lefts = self.get_lefts();
        print!("Remains:");
        for i in 0..self.metrics.get_max() {
            let idx: usize = i.try_into().unwrap();
            print!(" {}=>{}", i + 1, lefts[idx]);
        }
        println!();
    }

    /**
     * display debug info  (ex remaining possibles of the cells)
     **/
    pub fn debug(&mut self) {
        println!("-------------------------------DEBUG-------------------------------");
        let mut nb = 0;
        for i in 0..self.metrics.get_grid_size() {
            let pos: usize = i.try_into().unwrap();
            let cell: &mut Cell = &mut (self.cells[pos]);
            if cell.debug() {
                nb += 1;
            }
            if nb == self.metrics.get_square_side() {
                println!();
                nb = 0;
            }
        }
        if nb != self.metrics.get_square_side() {
            println!();
        }
        println!("-------------------------------DEBUG-------------------------------");
    }

    //grid decorations
    fn get_square_deco(&self, first: &str, fill: &str, sep: &str, last: &str) -> String {
        let mut res = String::new();
        res.push_str(first);
        for i in 0..self.metrics.get_square_side() {
            for _j in 0..self.metrics.get_square_side() {
                res.push_str(fill);
            }
            if i != self.metrics.get_square_side() - 1 {
                res.push_str(sep);
            }
        }
        res.push_str(last);
        res
    }
}

#[test]
fn deco_test() {
    let g = Grid::new(3);
    let first = g.get_square_deco("╔", "═══", "╦", "╗");
    assert_eq!(first, "╔═════════╦═════════╦═════════╗".to_string());
    let last = g.get_square_deco("╚", "═══", "╩", "╝");
    assert_eq!(last, "╚═════════╩═════════╩═════════╝".to_string());
    let mid = g.get_square_deco("╟", "═══", "╬", "╢");
    assert_eq!(mid, "╟═════════╬═════════╬═════════╢".to_string());

    let simple = g.get_square_deco("+", "---", "+", "+");
    assert_eq!(simple, "+---------+---------+---------+".to_string());
}

#[test]
fn display_test() {
    let mut g = Grid::new(3);
    g.display();
    g.set_val(1, 1, 1, CellType::Origin);
    g.set_val(1, 2, 2, CellType::Origin);
    g.set_val(1, 3, 3, CellType::Origin);
    g.set_val(1, 4, 4, CellType::Origin);
    g.set_val(1, 5, 5, CellType::Origin);
    g.set_val(1, 6, 6, CellType::Origin);
    g.set_val(1, 7, 7, CellType::Origin);
    g.set_val(1, 8, 8, CellType::Origin);
    g.set_val(1, 9, 9, CellType::Origin);
    g.set_val(2, 1, 4, CellType::Origin);
    g.set_val(2, 2, 5, CellType::Origin);
    g.set_val(2, 3, 6, CellType::Origin);
    g.set_val(2, 4, 7, CellType::Origin);
    g.set_val(2, 5, 8, CellType::Origin);
    g.set_val(2, 6, 9, CellType::Origin);
    g.set_val(2, 7, 1, CellType::Origin);
    g.set_val(2, 8, 2, CellType::Origin);
    g.set_val(2, 9, 3, CellType::Origin);
    g.set_val(3, 1, 7, CellType::Origin);
    g.set_val(3, 2, 8, CellType::Origin);
    g.set_val(3, 3, 9, CellType::Origin);
    g.set_val(3, 4, 1, CellType::Origin);
    g.set_val(3, 5, 2, CellType::Origin);
    g.set_val(3, 6, 3, CellType::Origin);
    g.set_val(3, 7, 4, CellType::Origin);
    g.set_val(3, 8, 5, CellType::Origin);
    g.set_val(3, 9, 6, CellType::Origin);
    g.set_val(4, 1, 2, CellType::Origin);
    g.set_val(4, 2, 3, CellType::Origin);
    g.set_val(4, 3, 4, CellType::Origin);
    g.set_val(4, 4, 5, CellType::Origin);
    g.set_val(4, 5, 6, CellType::Origin);
    g.set_val(4, 6, 7, CellType::Origin);
    g.set_val(4, 7, 8, CellType::Origin);
    g.set_val(4, 8, 9, CellType::Origin);
    g.set_val(4, 9, 1, CellType::Origin);
    g.set_val(5, 1, 5, CellType::Origin);
    g.set_val(5, 2, 6, CellType::Origin);
    g.set_val(5, 3, 7, CellType::Origin);
    g.set_val(5, 4, 8, CellType::Origin);
    g.set_val(5, 5, 9, CellType::Origin);
    g.set_val(5, 6, 1, CellType::Origin);
    g.set_val(5, 7, 2, CellType::Origin);
    g.set_val(5, 8, 3, CellType::Origin);
    g.set_val(5, 9, 4, CellType::Origin);
    g.set_val(6, 1, 8, CellType::Origin);
    g.set_val(6, 2, 9, CellType::Origin);
    g.set_val(6, 3, 1, CellType::Origin);
    g.set_val(6, 4, 2, CellType::Origin);
    g.set_val(6, 5, 3, CellType::Origin);
    g.set_val(6, 6, 4, CellType::Origin);
    g.set_val(6, 7, 5, CellType::Origin);
    g.set_val(6, 8, 6, CellType::Origin);
    g.set_val(6, 9, 7, CellType::Origin);
    g.set_val(7, 1, 3, CellType::Origin);
    g.set_val(7, 2, 4, CellType::Origin);
    g.set_val(7, 3, 5, CellType::Origin);
    g.set_val(7, 4, 6, CellType::Origin);
    g.set_val(7, 5, 7, CellType::Origin);
    g.set_val(7, 6, 8, CellType::Origin);
    g.set_val(7, 7, 9, CellType::Origin);
    g.set_val(7, 8, 1, CellType::Origin);
    g.set_val(7, 9, 2, CellType::Origin);
    g.set_val(8, 1, 6, CellType::Origin);
    g.set_val(8, 2, 7, CellType::Origin);
    g.set_val(8, 3, 8, CellType::Origin);
    g.set_val(8, 4, 9, CellType::Origin);
    g.set_val(8, 5, 1, CellType::Origin);
    g.set_val(8, 6, 2, CellType::Origin);
    g.set_val(8, 7, 3, CellType::Origin);
    g.set_val(8, 8, 4, CellType::Origin);
    g.set_val(8, 9, 5, CellType::Origin);
    g.set_val(9, 1, 9, CellType::Origin);
    g.set_val(9, 2, 1, CellType::Origin);
    g.set_val(9, 3, 2, CellType::Origin);
    g.set_val(9, 4, 3, CellType::Origin);
    g.set_val(9, 5, 4, CellType::Origin);
    g.set_val(9, 6, 5, CellType::Origin);
    g.set_val(9, 7, 6, CellType::Origin);
    g.set_val(9, 8, 7, CellType::Origin);
    g.set_val(9, 9, 8, CellType::Origin);
    g.display();
}

//methods to duplicate a grid
impl Grid {
    pub fn copy_from(&mut self, g: Grid) {
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

//methods to clone a grid
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
        let mut trace = String::new();
        trace.push_str(&self.trace);
        Grid {
            cells,
            acc: Accessor::new(self.metrics.get_square_side()), //Accessor always contains sames datas
            lines,
            columns,
            squares,
            resolved: self.resolved,
            debug: self.debug,
            metrics: self.metrics,
            trace,
        }
    }
}

#[test]
fn clone_grid_test() {
    let mut ori = Grid::new(3);
    ori.set_val(1, 1, 1, CellType::Origin);
    ori.set_val(2, 4, 1, CellType::Origin);
    ori.set_val(3, 7, 1, CellType::Origin);
    ori.set_val(4, 2, 1, CellType::Origin);
    ori.set_val(5, 5, 1, CellType::Origin);
    ori.set_val(6, 8, 1, CellType::Origin);
    ori.set_val(7, 3, 1, CellType::Origin);
    let mut copy = ori.clone();
    assert_eq!(ori.get_resolved(), copy.get_resolved());
    ori.set_val(8, 6, 1, CellType::Origin);
    assert_ne!(ori.get_resolved(), copy.get_resolved());
}

//methods to fill grid
impl Grid {
    pub fn compute_line(&mut self, line_number: u16, l: &str) {
        for (col, part) in l.split(',').enumerate() {
            let r: u16 = match part.parse() {
                Err(_) => {
                    continue;
                }
                Ok(v) => v,
            };
            let c: u16 = col.try_into().unwrap();
            self.set_val(line_number, c + 1, r, CellType::Origin);
        }
    }

    pub fn compute_vecline(&mut self, line_number: u16, vl: &[u16]) {
        let mut c = 1;
        for val in vl {
            if *val != 0 {
                self.set_val(line_number, c, *val, CellType::Origin);
            }
            c += 1;
        }
    }

    pub fn compute_vecvec(&mut self, vv: &[Vec<u16>]) {
        let mut l = 1;
        for v in vv {
            self.compute_vecline(l, v);
            l += 1;
        }
    }
}
