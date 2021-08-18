# rotateImage

### Problem Description

You are given an n x n 2D matrix that represents an image. Rotate the image by 90 degrees (clockwise).

### Example

- For

  ```
  a = [[1, 2, 3],
     [4, 5, 6],
     [7, 8, 9]]
  ```

  the output should be

  ```
  rotateImage(a) =
    [[7, 4, 1],
     [8, 5, 2],
     [9, 6, 3]]
  ```

### Input/Output

- **[execution time limit] 2 seconds (rs)**

- **[input] array.array.integer a**

  _Guaranteed constraints:_

  `1 ≤ a.length ≤ 100`,

  `a[i].length = a.length`,

  `1 ≤ a[i][j] ≤ 104.`

- **[output] array.array.integer**

### Solution Explanation

#### Solution #1

When looking at the expected output, we can identify that the first row becomes the last column (90 degree rotation).

We use this pattern to implement a solution which creates a new output variable and we apply this logic.

Where `i` is the row number, and `j` is the column number, `output[j]` becomes the inverse of `a[i]`.

This is an `O(n)` time-complexity solution, but requires `O(n)` space as well. We need to reduce the space used to make it an in-place solution.

#### Solution #2

By breaking down the problem into two steps, we can achieve an in-place `O(1)` space solution.

---

The first step is to "transpose" the matrix, so that we swap all `a[i][j]` with `a[j][i]`. This has the effect of making all the rows columns and the columns rows.

For example,

```
[[1, 2, 3],
 [4, 5, 6],
 [7, 8, 9]]
```

would become

```
[[1, 4, 7],
 [2, 5, 8],
 [3, 6, 9]]
```

We have swapped elements such as `2, 4` `3, 7` and `6, 8`.

---

Now, we can complete the second step: changing the order of the columns such that they are inversed.

To do this, we can swap `a[i][j]` with `a[i][a.len() - j - 1]`.

This has the effect of the following:

```
[1, 4, 7]
```

becomes

```
[7, 4, 1]
```

### Addendum

More Info: https://www.youtube.com/watch?v=IdZlsG6P17w
