# sudoku
To test algorithm solving sodoku

Sudoku is a grid like this:
------------------------------- 
| 1  0  0 | 0  0  0 | 0  0  0 |
| 0  0  0 | 2  0  3 | 0  0  0 |
| 0  0  0 | 0  0  0 | 0  0  3 |
------------------------------- 
| 0  0  4 | 0  0  0 | 0  0  0 |
| 0  0  0 | 0  3  0 | 0  6  0 |
| 0  6  0 | 0  0  1 | 0  3  0 |
-------------------------------
| 0  0  5 | 0  0  0 | 0  0  0 |
| 0  0  0 | 0  0  5 | 0  0  0 |
| 6  0  0 | 0  0  0 | 3  0  5 |
-------------------------------
wich could be solved as:
------------------------------- 
| 1  2  3 | 4  5  6 | 7  8  9 |
| 7  8  9 | 1  2  3 | 4  5  6 |
| 4  5  6 | 7  8  9 | 1  2  3 |
------------------------------- 
| 2  3  4 | 5  6  7 | 8  9  1 |
| 8  9  1 | 2  3  4 | 5  6  7 |
| 5  6  7 | 8  9  1 | 2  3  4 |
-------------------------------
| 3  4  5 | 6  7  8 | 9  1  2 |
| 9  1  2 | 3  4  5 | 6  7  8 |
| 6  7  8 | 9  1  2 | 3  4  5 |
-------------------------------

Accessor => some method to access to Row, Columns or squares
Cardinal => Enum to identify squares
Cell => a cell with eventually the answer (number 1 to 9) or the possibles answer if not yet solved
Grid => the 3x3 squares each made of 3x3 cells. So Grid is 9x9 cells (in a Vec of 81 elemens)
Resolve => some methode to solve the sudoku

Mapping:
position of Cells in the Vec:
All positions: : 
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
| 9 |10 |11 |12 |13 |14 |15 |16 |17 |
|18 |19 |20 |21 |22 |23 |24 |25 |26 |
|27 |28 |29 |30 |31 |32 |33 |34 |35 |
|36 |37 |38 |39 |40 |41 |42 |43 |44 |
|45 |46 |47 |48 |49 |50 |51 |52 |53 |
|54 |55 |56 |57 |58 |59 |60 |61 |62 |
|63 |64 |65 |66 |67 |68 |69 |70 |71 |
|72 |73 |74 |75 |76 |77 |78 |79 |80 |

position of the Rows (and cells in them)
1-> 0, 1, 2, 3, 4, 5, 6, 7, 8
2-> 9,10,11,12,13,14,15,16,17
3->18,19,20,21,22,23,24,25,26
4->27,28,29,30,31,32,33,34,35
5->36,37,38,39,40,41,42,43,44
6->45,46,47,48,49,50,51,52,53
7->54,55,56,57,58,59,60,61,62
8->63,64,65,66,67,68,69,70,71
9->72,73,74,75,76,77,78,79,80

position of the Columns (and cells in them)
  1   2   3   4   5   6   7   8   9
  |   |   |   |   |   |   |   |   |
  \/  \/  \/  \/  \/  \/  \/  \/  \/
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
| 9 |10 |11 |12 |13 |14 |15 |16 |17 |
|18 |19 |20 |21 |22 |23 |24 |25 |26 |
|27 |28 |29 |30 |31 |32 |33 |34 |35 |
|36 |37 |38 |39 |40 |41 |42 |43 |44 |
|45 |46 |47 |48 |49 |50 |51 |52 |53 |
|54 |55 |56 |57 |58 |59 |60 |61 |62 |
|63 |64 |65 |66 |67 |68 |69 |70 |71 |
|72 |73 |74 |75 |76 |77 |78 |79 |80 |

position of the square:
1 2 3 
4 5 6
7 8 9

Name of the squares in the Enum:
 NW    N   NE
  W    C   E
  SW   S   SE

Cells of the squares:
1/NW =>  0, 1, 2, 9,10,11,18,19,20
2/N  =>  3, 4, 5,12,13,14,21,22,23
3/NE =>  6, 7, 8,15,16,17,24,25,26
4/W  => 27,28,29,36,37,38,45,46,47
5/C  => 30,31,32,39,40,41,48,49,50
6/E  => 33,34,35,42,43,44,51,52,53
7/SW => 54,55,56,63,64,65,72,73,74
8/S  => 57,58,59,66,67,68,75,76,77
9/SE => 60,61,62,69,70,71,78,79,80


Resolution lvl 0:
  a cell is resolved when there only one possible left (At start there is 9 possible values for each cell)

Resolution lvl 1:
  If a cell is resolved then his value is in no other cells of the same Row, in no other cells of the same 
  column and in no other cells of the same square
exemple 
   cell 0 is 1
   value 1 can't be in others cells of line 1 (1,2,3,4,5,6,7,8)
   value 1 can't be in others cells of column 1 (9,18,27,36,45,54,63,72)
   value 1 can't be in others cells of square NW (1,2,9,10,11,18,19,20)


Resolution lvl 2:
  if the same value is resolve in left, right, up and down square of ours we can identify where it should be
example:
------------------------------- 
| 1  0  0 | 0  0  0 | 0  0  0 |
| 0  0  0 | 1  0  0 | 0  0  0 |
| 0  0  0 | 0  0  0 | 0  0  0 |
------------------------------- 
| 0  0  0 | 0  0  0 | 1  0  0 |
| 0  0  0 | 0  0  0 | 0  0  0 |
| 0  0  0 | 0  0  1 | 0  0  0 |
-------------------------------
| 0  1  0 | 0  0  0 | 0  0  0 |
| 0  0  0 | 0  1  0 | 0  0  0 |
| 0  0  0 | 0  0  0 | 0  0  1 |
-------------------------------
we know were is 1 in squares NW & N -> it is in row 1 and 2 so in square NE it must be in row 3
we know were is 1 in squares E & SE -> it is in column 1 and 3 so in square NE it must be in column 2
-> in square NE the cell at line 3 and column 2 has the value 1 -> Cel N° 25

lvl 3:
If a value is not in the possible of a line less a cell then the cell has this values
same for column
same for square


lvl4: x wing to dow
if a value is present in two columns in only two same lines 
then it can't be elsewhere in the same lines
example
------------------------------- 
| 6  7  9 | 4  1  8 | 3  5  2 |
| 2  4  8 | 3  9  5 | 7  6  1 |
| 1  5  3 | 7  6  2 | 9  8  4 |
------------------------------- 
| x  0  0 | 0  0  0 | 4  2  x |
| 7  0  0 | 0  8  0 | 5  3  6 |
| x  0  0 | 0  0  0 | 1  7  x |
-------------------------------
| 5  8  7 | 1  2  9 | 6  4  3 |
| 4  6  1 | 8  0  7 | 2  0  5 |
| 3  0  2 | 0  5  4 | 8  1  7 |
-------------------------------
in column 0, 9 can be in line 4 or 6
in column 8, 9 can be in line 4 or 6
So 9 can't be in columns 1,2,3,4,5 of line 4 and in columns 1,2,3,4,5 of line 6

by symetry:
if a value is present in two lines in only two same columns 
then it can't be elsewhere in the same columns
-> TODO


lvl 9: try a guess
when nothing more can be found with levels 0,1,2,3,4
then we choose a cell and try to guess the value
if exists Xwing candidates we use them else we use a cell with minimum number of possible values
from the possibles of this cell we try firts the one which is more present in solved cells
We try a guess for this cell
then we run again other levels of resolution
if at some point we get an error then the guess is bad so we eliminate this value 
of the possibles for this cell
Obviously this is a recursive way of resolution



other lvl to find...
....