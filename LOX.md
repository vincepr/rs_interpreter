# Defining the target language
Features of the Lox language, for a more comprehensive chapter check the book.

- Dynamic typing
- automatic memory management (using reference counting)

## data types
- Booleans: with `true || false`
- Numbers: with int`123` and float `12.3` under the hood
- Strings: `"whatever"`and the empty string`""`
- Nil: `nil` the null implementaion for a no value.

## Expressions
### Arithmetic operations. 
If anything but a number is passed into them it should fail. (exception + string concatenation)
```
add + me;
subtract - me;
multiply * me;
divice / me;

- negateMe;
```
### Comparison and equality. 
Must always return a Boolean.
```
less < than;
lesThan <= orEqual;
greater > than;
greaterThan >= orEqual;
```
### Logical operators
```
!true           //false
!false          //true

true and false; //false
true and true;  //true

false or false; //false
true or false;  //true
```
### Precedence and grouping
Should have same Precedence as in Math (Priority)

### Didnt make the cut:
bitewise shift, shift, modulo, conditional operators...

## Statements
Statements produce a *side effect*. (while expressions produce a value).

```
print "some expression";    // print out to std-out
clock();                    // the number of seconds till execution start of the programm

// possible to wrap multiple statements in a block
{
    print"One";
    print"Two";
}
```

## Variables
```
var isNil // ->nil
var count = 0;
var greet = "some string";
print greet;
count = count +1;
```

## Control Flow
```
// if statement
if (condition) {
    print"yes";
} else {
    print"no";
}

// loops:
var count = 0;
while (a<10){
    a = a+1;
    print a;
}

// for loops
for (var a = 1; a < 10; a = a +1 ){
    print a;
}
```

## Functions
Should behave just like expected:
```
fun printSum(a,b){
    print a + b;
}
printSum(bacon, 3);
doStuff();
```
- Closures/ First class functions are in Lox. So you should be able to pass them arround.
```
// so this should work
fun add(a, b){
    return a+b;
}
fun identity(a){
    return a;
}
print identity(add)(1,2)    // -> "3"
```
-  local functions inside blocks
```
fun outer(){
    fun inner(){
        print "is local";
    }
    inner();
}
```

## Classes
```
class Breakfast{
    init(meat){
        this.meat = meat;
        this.bread = "default bread";
    }

    // methods
    cook(){
        print "do cooking";
    }
    serve(table){
        print "Bringing"+ this.meat +" to table "+ table +", enjoy the meal.";
    }
}

// initialize an instance:
var b = Breakfast("ham");
print b;    // "Breakfast instance"
b.cook();

// freely able to add fields to an instance:
b.meat = "sausage";
b.bread = "breadrolls";
```

### Inheritance
Single inheritance using `<` operator
```
class Brunch < Breakfast{
    init(meat, drink){
        super.init(meat);
        this.drink = drink;
    }

}
```