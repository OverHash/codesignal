# sudoku2

### Problem Description

_Sudoku_ is a number-placement puzzle. The objective is to fill a `9 × 9` grid with numbers in such a way that each column, each row, and each of the nine `3 × 3` sub-grids that compose the grid all contain all of the numbers from `1` to `9` one time.

Implement an algorithm that will check whether the given `grid` of numbers represents a valid _Sudoku_ puzzle according to the layout rules described above. Note that the puzzle represented by `grid` does not have to be solvable.

### Example

- For

  ```
  grid = [['.', '.', '.', '1', '4', '.', '.', '2', '.'],
        ['.', '.', '6', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '1', '.', '.', '.', '.', '.', '.'],
        ['.', '6', '7', '.', '.', '.', '.', '.', '9'],
        ['.', '.', '.', '.', '.', '.', '8', '1', '.'],
        ['.', '3', '.', '.', '.', '.', '.', '.', '6'],
        ['.', '.', '.', '.', '.', '7', '.', '.', '.'],
        ['.', '.', '.', '5', '.', '.', '.', '7', '.']]
  ```

  the output should be

  `sudoku2(grid) = true`;

- For

  ```
  grid = [['.', '.', '.', '.', '2', '.', '.', '9', '.'],
        ['.', '.', '.', '.', '6', '.', '.', '.', '.'],
        ['7', '1', '.', '.', '7', '5', '.', '.', '.'],
        ['.', '7', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '8', '3', '.', '.', '.'],
        ['.', '.', '8', '.', '.', '7', '.', '6', '.'],
        ['.', '.', '.', '.', '.', '2', '.', '.', '.'],
        ['.', '1', '.', '2', '.', '.', '.', '.', '.'],
        ['.', '2', '.', '.', '3', '.', '.', '.', '.']]
  ```

  the output should be

  `sudoku2(grid) = false`.

  The given grid is not correct because there are two `1`s in the second column. Each column, each row, and each `3 × 3` subgrid can only contain the numbers `1` through `9` one time.

### Input/Output

- **[execution time limit] 2 seconds (rs)**

- **[input] array.array.char grid**

  A `9 × 9` array of characters, in which each character is either a digit from `'1'` to `'9'` or a period `'.'`.

- **[output] boolean**

  Return `true` if `grid` represents a valid _Sudoku_ puzzle, otherwise return `false`.

### Solution Explanation

#### Solution #1

We break this down into three problems:

1. **Validating sub-grids**

   We need to validate each small 3x3 grid.

   We can do so by iterating from 0->3 on the y-axis and 0->3 on the x-axis. We are then inside a "sub-grid" and we can store a HashSet of all the seen values and then iterate from 0->9 through the elements and check if they are in the HashSet or not. If they are, then it is a duplicate and we return false.

2. **Validating rows**

   We can iterate through each row in the Vector, store a HashSet of seen values, and then iterate through each element in the Vector to see if we have seen it. If we have, it is a duplicate and we return false.

3. **Validating columns**

   We can iterate from 0->9 along the x-axis, storing seen elements in a HashSet and then an inner loop from 0->9 for the y-axis and access the grid through `grid[y][x]` to see if we have seen an element. If we have, it is a duplicate and we return false.

This solution is an `O(1)` time-complexity solution, because we iterate a constant amount of times, and an `O(1)` space-complexity solution because we do not store auxiliary data. Note that if we were to make an N by N board, this solution is an `O(n)` time-complexity and `O(n)` space-complexity.

### Solution 2

We can improve our performance by iterating less and having a simpler solution by manipulating our HashSet to store a string instead of the chars.

If we stored one HashSet that contains values such as `"5 found in row 0"`, `"6 found in row 2"` we can have an optimal solution.

This works because HashSets do not store duplicates, so we can look-up and see if we have already stored a value in our row/column/sub-box.

### Addendum

More Info: https://www.youtube.com/watch?v=Pl7mMcBm2b8
