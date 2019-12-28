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
  a cell is resolved when there only one possible left (At start ther is 9 possible values for each cell)

Resolution lvl 1:
  If a cell is resolved then his number is in no other cells of the same Row, in no other cells of the same column and in no other cells of the same square

Resolution lvl 2:
  if the same value is resolve in left, right, up and down square of ours we can identify where it should be
exemple:
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
-> in square NE 1 is in the cell at line 3 and column 2 -> Cel N° 25

other lvl to find...

We call lvl 0 until ther is no more change
   Then we call lvl 1 until there is no more change
then we go back to lvl 0 and 1 until there is no more change
      Then we call lvl 2 until there is no more change
then we go back to lvl 0, 1 and 2 until there is no more change
....