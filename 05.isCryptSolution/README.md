# isCryptSolution

### Problem Description

A _cryptarithm_ is a mathematical puzzle for which the goal is to find the correspondence between letters and digits, such that the given arithmetic equation consisting of letters holds true when the letters are converted to digits.

You have an array of strings `crypt`, the _cryptarithm_, and an an array containing the mapping of letters and digits, `solution`. The array `crypt` will contain three non-empty strings that follow the structure: `[word1, word2, word3]`, which should be interpreted as the `word1 + word2 = word3` cryptarithm.

If `crypt`, when it is decoded by replacing all of the letters in the cryptarithm with digits using the mapping in `solution`, becomes a valid arithmetic equation containing no numbers with leading zeroes, the answer is `true`. If it does not become a valid arithmetic solution, the answer is `false`.

Note that number `0` doesn't contain leading zeroes (while for example `00` or `0123` do).

### Example

- For `crypt = ["SEND", "MORE", "MONEY"]` and

  ```
  solution = [['O', '0'],
            ['M', '1'],
            ['Y', '2'],
            ['E', '5'],
            ['N', '6'],
            ['D', '7'],
            ['R', '8'],
            ['S', '9']]
  ```

  the output should be `isCryptSolution(crypt, solution) = true`.

  When you decrypt `"SEND"`, `"MORE"`, and `"MONEY"` using the mapping given in crypt, you get `9567 + 1085 = 10652` which is correct and a valid arithmetic equation.

- For

  For `crypt = ["TEN", "TWO", "ONE"]` and

  ```
  solution = [['O', '1'],
            ['T', '0'],
            ['W', '9'],
            ['E', '5'],
            ['N', '4']]
  ```

  the output should be `isCryptSolution(crypt, solution) = false`.

  Even though `054 + 091 = 145`, `054` and `091` both contain leading zeroes, meaning that this is not a valid solution.

### Input/Output

- **[execution time limit] 2 seconds (rs)**

- **[input] array.string crypt**

  An array of three non-empty strings containing only uppercase English letters.

  _Guaranteed constraints_:

  `crypt.length = 3`,

  `1 ≤ crypt[i].length ≤ 14`.

- **[input] array.array.char solution**

  An array consisting of pairs of characters that represent the correspondence between letters and numbers in the cryptarithm. The first character in the pair is an uppercase English letter, and the second one is a digit in the range from `0` to `9`.
  
  It is guaranteed that `solution` only contains entries for the letters present in `crypt` and that different letters have different values.

  _Guaranteed constraints_:

  `solution[i].length = 2`,

  `'A' ≤ solution[i][0] ≤ 'Z'`,

  `'0' ≤ solution[i][1] ≤ '9'`,

  `solution[i][0] ≠ solution[j][0], i ≠ j`,

  `solution[i][1] ≠ solution[j][1], i ≠ j`.

- **[output] boolean**

  Return `true` if the `solution` represents the correct solution to the cryptarithm `crypt`, otherwise return `false`.

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

This solution is an `O(1)` time-complexity solution, because we iterate a constant amount of times, and an `O(1)` space-complexity solution because we do not store auxiliary data. Note that if we were to make an `n` by `n` board, this solution is an `O(n^2)` time-complexity and `O(n)` space-complexity.

### Solution 2

We can improve our performance by iterating less and having a simpler solution by manipulating our HashSet to store a string instead of the chars.

If we stored one HashSet that contains values such as `"5 found in row 0"`, `"6 found in row 2"` we can have an optimal solution.

This works because HashSets do not store duplicates, so we can look-up and see if we have already stored a value in our row/column/sub-box.

Note that this solution is likely slower, due to string hashing taking a long amount of time. It is good to know to think outside the box though.

This solution is also `O(n^2)` where `n` is the grid-length (in this case, `9`).

### Addendum

More Info: https://www.youtube.com/watch?v=Pl7mMcBm2b8
