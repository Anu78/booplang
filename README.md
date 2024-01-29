# english -> programming

## sample syntax
```
func fib uses x, y, z
    if x lt 3 then return 1
    else return fib(x-1) + fib(x-2)
done

x is 40
fib(x)
```
This is some sample source code. The exact wording and syntax is not final, but the general idea is that the language should allow the programmer to type as fast as possible.

## the main goal

The main goal of this project is to make a language that reads almost like english (so that you can type it out as if you were writing an essay, without needing parenthesis or any other difficult to reach keys on the keyboard). It will be compiled ahead of time, and should have enough built in features to solve most leetcode problems. (maps, sets, 2d arrays, printing, etc).

## some notes
    1. the return types of functions will be inferred
    2. the return types of variables will be inferred (if they are not specified)
