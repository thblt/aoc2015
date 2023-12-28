This is a fun problem.  A few things must be noticed first.

```
   | 1   2   3   4   5   6
---+---+---+---+---+---+---+
 1 |  1   3   6  10  15  21
 2 |  2   5   9  14  20
 3 |  4   8  13  19
 4 |  7  12  18
 5 | 11  17
 6 | 16
```

 - We refer to cells by (row,col).  (5,2) is 17.

 - We fill diagonally, but the size of a diagonal is always its height
   and width.  The first diagonal (started in row 1) is of size 1, the
   second of size 2, the third of size 3, and so on.  Thus, the number
   of each row is the length of its diagonal.

 - It follows that the number in column 1 of row N is the sum of (1..N-1) + 1

> sum1N :: Int -> Int
> sum1N n  = (n ^ 2 `div` 2) + (n `div` 2) + diff
>   where diff = if even n then 0 else 1

 - For each row R, the value at column C belongs in the diagonal that
   starts row (R+C)-1.

Our input:

> row = 3010
> col = 3019

We're in a diagonal that started at

> diagStartRow = (row + col) - 1
> diagStartVal = (sum1N (diagStartRow-1)) + 1

> nth = diagStartVal + (diagStartRow - row)

> compute :: Int -> Integer -> Integer
> compute 1 p = p
> compute n p = compute (n-1) ((p * 252533) `mod` 33554393)


> main = putStrLn . show $ compute nth 20151125
