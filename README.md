# tom-language README

This is a VS Code extension for Tom language support in VS Code.

At first, it will only support syntax highlighting, but full language support is the goal.
# TODO
* What does "customize" mean?  
    - Extend?
    - The meaning of "customize" in various contexts
* Glossary
    - 要素　(Element; shape, line etc.)

## About the TOM programming language
* The TOM language was made for customizing the CAD software Jissun Boushi (実寸法師)
* Tom is short for Taiwa Object Module
* It is a very simple language with syntax similar to C++ and Java
* Module files (\*.tom) are created by compiling module source files (\*.toms)
* Module files are excecuted using Jissun Boushi

### Language structure
* Modules are made up of collections of Classes
* Classes are made up of Members
* There are three types of Members: Functions, Variables, and Constants
* Programming is performed by customizing built in classes and defining new classes

### Reserved words/keywords used in Modules
* `title "<module name>";`  
    - This is the name of the module
    - This is displayed in the menu when excecuting
    - Without this, the module will not run

* `author "author name";`  
	- The name of the module author  
	- This is displayed as a property of the module
* `native "native module name";`
	- Specifies a DLL that contains native functions written in C
	- Not commonly used
* `attach "filename";`
	- Used to attach files to the module file
* `class`
	- Defines a class
#### *** `title`, `author`, and `native` can only appear once in a module. (Except for the class name)

### author
* The author name can be included in the module file
* The author keyword has no other meaning than to display the author name in the properties window
* Author names can be freely added but the following best practices are recommended
    * Choose a simple codename that is not easily confused with others
    * Do not use copyrighted content
    * When modifying someone else's program to a great degree, change to your own name
    * If modifying only one part of someone else's program, include your name in the author property as follows  
        `author <original author name>, <your name>;`

### Comments
* Comments can be inserted anywhere in the source file
* Two forward slashes (//) indicates the start of a comment until the end of the line
* Multiple lines can be commented using /* ... \*/  
Everything in between /* and \*/ is commented
Multiple line comments cannot be nested

### Class Definition
```
<public/private> <final> class new_class_name <extends parent_class>
```
* If you omit the parent class name, it becomes an Object class. (When omitted, the extends keyword is also omitted.) TODO: 意味不明
* Classes not declared public or private are public by default
* If declared final, a class cannot be customized

#### Actual class definition example
```
title "Class Title": // Displayed in the menu during execution. Necessary for execution
// It's best not to assign a title to classes not meant to be run
```

#### Runnable classes
* Runnable classes are derived directly or indirectly from Scenario or Executable classes
* For functions that specify points and select elements, customize the Scenario class 
* For functions that can be processed just by displaying dialogs, etc, customize the Executable class 

### Member definitions
* Defining a member constant (Also referred to as a class constant)  
        ```
            <public> const <type> <constant_name> = <value>;
        ```  
※ Possible types are string, int, and double only  
※ Constants of type class or array are not possible  
※ Constants are public by default so the public keyword is usually omitted

#### Member variables
`<public/private> <static> <type> <variable_name> = <value>;`  
※ private by default  
※ static variables are class variables. Non-static variables are instance variables  
※ variables of the same type can be defined together in one line   
`<type> var_x = <value>, var_y = <value>, ...;`  
※ Arrays are initialized within an initialization block defined by curly braces  
`{<value_1>, <value_2>, ...}`  
※ The difference between class variables and instance variables will be described later on

#### Member functions
* Class functions  
``` <public/private> static <return type> <function name>(parameter1, parameter2, ...) <function>```
* Instance functions  
```<public/private> <final/abstract> <return type> <function name> (parameter1, parameter2, ...) <function>```  
    * Classes set as final cannot be customized later
    * Member variables of final classes are all final by default
    * Abstract classes must be customized in order to be used

* Member functions are public by default
* Functions are written in executable blocks defined by curly braces `{...}`  
And statements are terminated by semi-colons `;`  
* Functions that do not contain an executable block do nothing and return `null`  
This is only useful for defining abstract classes
* Reserved keywords for native variables also exist

#### Executable blocks  
* Executable blocks contain local variables and statements
* Local variables are only valid within the scope of the current block
* Local variables cannot be defined as `public\private\static`  
Otherwise, they are defined the same way as member variables  

###### Legend for the following executable statement examples
`A` ー an expression such as `myFunc(param1, param2)`  
`B` ー a statement or executable block (`{func1(); func2(); x = 3; etc...}`)  
`C` ー a constant (In this case used to represent cases in a switch statement)  
`L` ー A label TODO: ??

```
A;                  // simple executable statement
B                   // nested executable block
                    (In this case B = {some code...})
if (E) B            // if statement
if (E) B else B     // if-else statement
L: switch (A) B     // switch-case statement
                    (Switch statements must be accompanied by case: and default:)
    case C:         // case label
    default:        // default label
L: for (A; A; A) B  // for-loop

```

#### Expressions and Statements
* An expression can be the value of a constant, variable, function, etc. or a mathematical operation on any of the former.  
* Statements do not contain values. Expressions contain values.
* If a statement consists of just an expression and nothing else, the value of the expression is lost
* The value of a statement must be assigned using an operator for it to be useful
* The only expressions that can be used as simple execution statements are simple assignment expressions (=), arithmetic assignment expressions (+=, etc.), increase/decrease expressions (++, --), function call expressions, and 
object generation expressions, which are simple assignment statements, arithmetic assignment statements, increase/decrease statements, function call statements, and 
object generation statements, respectively.
* Although an object generation statement may seem meaningless since the object disappears immediately after execution, 
at least the constructor is invoked, so it is classified as the same as a function call statement.
## Features

Describe specific features of your extension including screenshots of your extension in action. Image paths are relative to this README file.

For example if there is an image subfolder under your extension project workspace:

\!\[feature X\]\(images/feature-x.png\)

> Tip: Many popular extensions utilize animations. This is an excellent way to show off your extension! We recommend short, focused animations that are easy to follow.

## Requirements

If you have any requirements or dependencies, add a section describing those and how to install and configure them.

## Extension Settings

Include if your extension adds any VS Code settings through the `contributes.configuration` extension point.

For example:

This extension contributes the following settings:

* `myExtension.enable`: Enable/disable this extension.
* `myExtension.thing`: Set to `blah` to do something.

## Known Issues

Calling out known issues can help limit users opening duplicate issues against your extension.

## Release Notes

Users appreciate release notes as you update your extension.

### 1.0.0

Initial release of ...

### 1.0.1

Fixed issue #.

### 1.1.0

Added features X, Y, and Z.

---

## Working with Markdown

You can author your README using Visual Studio Code. Here are some useful editor keyboard shortcuts:

* Split the editor (`Cmd+\` on macOS or `Ctrl+\` on Windows and Linux).
* Toggle preview (`Shift+Cmd+V` on macOS or `Shift+Ctrl+V` on Windows and Linux).
* Press `Ctrl+Space` (Windows, Linux, macOS) to see a list of Markdown snippets.

## For more information

* [Visual Studio Code's Markdown Support](http://code.visualstudio.com/docs/languages/markdown)
* [Markdown Syntax Reference](https://help.github.com/articles/markdown-basics/)

**Enjoy!**
