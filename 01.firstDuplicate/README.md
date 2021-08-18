# firstDuplicate

### Problem Description

Given an array `a` that contains only numbers in the range from `1` to `a.length`, find the first duplicate **number** for which the second occurrence has the minimal index. In other words, if there are more than 1 duplicated numbers, return the **number** for which the second occurrence has a smaller index than the second occurrence of the other number does. If there are no such elements, return `-1`.

### Example

- For `a = [2, 1, 3, 5, 3, 2]`, the output should be `firstDuplicate(a) = 3`.

  - There are `2` duplicates: numbers `2` and `3`. The second occurrence of `3` has a smaller index than the second occurrence of `2` does, so the answer is `3`.

- For `a = [2, 2]`, the output should be `firstDuplicate(a) = 2`;

- For `a = [2, 4, 3, 5, 1]`, the output should be `firstDuplicate(a) = -1`.

### Input/Output

- **[execution time limit] 2 seconds (rs)**

- **[input] array.integer a**

  Guaranteed constraints:

  `1 ≤ a.length ≤ 105`,

  `1 ≤ a[i] ≤ a.length`.

- **[output] integer**

  The element in `a` that occurs in the array more than once and has the minimal index for its second occurrence. If there are no such elements, return `-1`.

### Solution Explanation

#### Solution #1

The idea here is to use a variable to store the minimum index of the _second_ element that we have found (the duplicate). This is necessary because we need to look for the first occurrence of the second duplicate in the array, not the first element that has a duplicate in the array somewhere.

This is an `O(n^2)` time complexity solution with `O(1)` space because we iterate through every other element, so it is not fast enough.

#### Solution #2

We can use space complexity to increase time-complexity.

In this case, we can use a HashSet to make our solution `O(n)` in time and `O(n)` in space by storing the value each time we see it. If the set already contains the value, then it is the second duplicate and we can return.

Although this solution is fast enough, it uses up space as well.

#### Solution #3

The optimal solution will use `O(n)` time complexity and `O(1)` space complexity.

It is important to note that in the criteria we were told that integers in array are between `1` and `a.length`. This is important because now we can use the values in the array as indexes.

The trick here is that we can loop through the array going from start to end, index `a[Math.Abs(n) - 1]` and make it a negative number. If it already was a negative number, then we have already seen this number, so we can therefore return `Math.Abs(n)`.

It is important to note that this solution mutates the input.

### Addendum

More Info: https://www.youtube.com/watch?v=XSdr_O-XVRQ
