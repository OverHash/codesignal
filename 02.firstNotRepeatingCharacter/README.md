# firstNotRepeatingCharacter

### Problem Description

Given a string `s` consisting of small English letters, find and return the first instance of a non-repeating character in it. If there is no such character, return `'_'`.

### Example

- For `s = "abacabad"`, the output should be
  `firstNotRepeatingCharacter(s) = 'c'`.

  There are `2` non-repeating characters in the string: `'c'` and `'d'`. Return `c` since it appears in the string first.

- For `s = "abacabaabacaba"`, the output should be
  `firstNotRepeatingCharacter(s) = '_'`.

  There are no characters in this string that do not repeat.

### Input/Output

- **[execution time limit] 2 seconds (rs)**

- **[input] string s**

  A string that contains only lowercase English letters.

  _Guaranteed constraints:_

  `1 ≤ s.length ≤ 10^5.`,

- **[output] char**

  The first non-repeating character in `s`, or `'_'` if there are no characters that do not repeat.

### Solution Explanation

#### Solution #1

Here we iterate through the string, storing a boolean if we have found a duplicate on the way. We then do a second iteration again through the string and check if the characters are the same (and the indexes are not). If they are the same, then it is a duplicate, so we flip the duplicate boolean we stored earlier.
After the second iteration, if the duplicate flag was not triggered, we can return the string.

This is an `O(n^2)` time complexity solution with `O(1)` space because we iterate through every other element, so it is not fast enough.

#### Solution #2

We can use space complexity to reduce our time-complexity.

In this case, we store a HashMap where the key is the character, and the value is the amount of times we have seen the character. We iterate through each character once, adding it to the map and incrementing if there is a value already there.

Once we have iterated through the string once, we iterate again and lookup the character. If the occurrence count is `1`, there is no duplicates and we can therefore return the character.

This is a `O(n)` time-complexity solution because we do two iterations of the string, and an `O(n)` space-complexity because our HashMap stores up to `n` characters.

_Note: one could argue that this is an `O(1)` space-complexity solution due to there being a finite amount of characters. This algorithm could be applied to integers as well, making it more like `O(n)` in some practical applications._

### Addendum

More Info: https://www.youtube.com/watch?v=5co5Gvp_-S0
