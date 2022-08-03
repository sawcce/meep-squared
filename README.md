# Meep²
Built to be easy to pickup, made to be versatile (Truttle1's Practical Language Jam 2022 participation)

# Premices
Meep² is an interpreted language that follow these principles:
- The programs should be verbose and small
- Parsing, Variable resolution and macros are computed JIT (right before the program starts executing)
- No hidden exceptions, everything should be safe
- References shouldn't be unsafe to work with

# Base syntax
Meep² is simple: it just expects a list of statements  

But what is a statement ? It's:
- A variable assignement
- A function definition
- A function call
- A type definition

## Variable assignement
```
name_0123456789 = value
```

## Function call
```
function_name(args)
```

## A function definition
```
function_name closure
```

Wait what's a closure ?  

It's just an anonymous function, a function with no name the syntax is the following

```
# Without arguments
# Either
-> function_call or value
# Or
_ -> function_call or value

# With an argument
x -> x+1
#     ^ See here you don't need a return statement in this basic closure, this will evaluate to x + 1

# Multiple statements
x -> 
    print(x)
    x + 1
end
# Same as before you only need to put the return value at the end with no return statement

# With multiple arguments
x, y -> 2x + y
```
