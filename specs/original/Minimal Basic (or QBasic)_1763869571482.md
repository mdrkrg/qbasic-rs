# Minimal Basic (or QBasic)
<span style="color: red;">**Please compress your source code using 7z and upload the 7z file to oc.sjtu.edu.cn before the final deadline.**</span>
![qbasic-fig1](https://notes.sjtu.edu.cn/uploads/upload_d170cf4333ec6a0b42767e3528ecc2e7.png)
In 1975, Bill Gates and Paul Allen started the company that would become Microsoft by writing a BASIC interpreter for the first microcomputer, the Altair 8800 developed by the MITS corporation of Albuquerque, New Mexico. By making it possible for users to write programs for a microcomputer without having to code in machine language, the Altair and its implementation of BASIC helped to start the personal computer revolution.

In this project, your mission is to build a minimal BASIC interpreter. You need to accomplish the following objectives:
- To increase your familiarity with expression trees and class inheritance.
- To give you a better sense of how programming languages work. Learning how an interpreter operates—particularly one that you build yourself—provides useful insights into the programming process.
- Enhance the understanding of compilation and learn how to translate high-level languages into lower-level assembly instructions that correspond one-to-one with machine code.
- To offer you the chance to adapt an existing program into one that solves a different but related task. The majority of programming that people do in the industry consists of modifying existing systems rather than creating them from scratch. 

## 1. What is Basic?

The programming language BASIC—the name is an acronym for Beginner’s All-purpose Symbolic Instruction Code—was developed in the mid-1960s at Dartmouth College by John Kemeny and Thomas Kurtz. It was one of the first languages designed to be easy to use and learn. Although BASIC has now pretty much disappeared as a teaching language, its ideas live on in Microsoft’s Visual Basic system, which remains in widespread use. 

In BASIC, a program consists of a sequence of numbered statements, as illustrated by the simple program below:
```BASIC
10 REM Program to add two numbers
20 INPUT n1
30 INPUT n2
40 LET total = n1 + n2
50 PRINT total
60 END
```
The line numbers at the beginning of the line establish the sequence of operations in a program. In the absence of any control statements to the contrary, the statements in a program are executed in ascending numerical order starting at the lowest number. Here, for example, program execution begins at line 10, which is simply a comment (the keyword REM is short for REMARK) indicating that the purpose of the program is to add two numbers. Lines 20 and 30 request two values from the user, which are stored in the variables n1 and n2, respectively. The LET statement in line 40 is an example of an assignment in BASIC and sets the variable total to be the sum of n1 and n2. Line 50 displays the value of total on the console, and line 60 indicates the end of execution. A sample run of the program therefore looks like this:

![qbasic-fig2](https://notes.sjtu.edu.cn/uploads/upload_e043d3debdd9bdb83c655514cbca5074.png)
*Figure1*

Line numbers are also used to provide a simple editing mechanism. Statements need not be entered in order, because the line numbers indicate their relative position. Moreover, as long as the user has left gaps in the number sequence, new statements can be added in between other statements. For example, to change the program that adds two numbers into one that adds three numbers, you would need to make the following changes:

1.	Add a new line to read in the third value by typing in the command
```BASIC
35 INPUT n3
```
2. This statement is inserted into the program between line 30 and line 40. Replace the old line 40 with an update version by typing
```
40 LET total = n1 + n2 + n3
```

In classical implementations of BASIC, the standard mechanism for deleting lines was to type in a line number with nothing after it on the line. Note that this operation actually deleted the line and did not simply replace it with a blank line that would appear in program listings.

### 1.1. Expressions in BASIC
The `LET` statement illustrated by line 40 of the addition program has the general form
```BASIC
LET variable = expression
```
and has the effect of assigning the result of the expression to the variable. In Minimal BASIC, **the assignment operator is no longer part of the expression structure**. The simplest expressions are variables and integer constants. These may be combined into larger expressions by enclosing an expression in parentheses or by joining two expressions with the operators **+**,**-**, **\***, **/** and **MOD**. <span style='color: blue;'>You only need to support +, -, *, /, MOD, (, ) operators with signed integers (at least 32-bit) in expressions. (Be aware of negative integers.)</span>

The MOD operator has the same precedence as * and /. In the expression LET r = a MOD b, the absolute value of r should be less than the absolute value of b, and the sign of r is the same as that of b. For example, 5 MOD 3 is 2 and 5 MOD (-3) is -1.

Additionally, you need to support the exponentiation operator in expressions:
```BASIC
exp1 ** exp2
```
The exponentiation operator returns the result of exp1exp2, where exp1 and exp2 are expressions. The exponentiation operator is right associative, i.e., a ** b ** c is equal to a ** (b ** c). The operator has higher precedence than *, / and MOD.

<span style='color: blue;'>For all expressions and statements, you need to handle extra spaces. For example, LET a   = b + 4 *    (-5 + 4 ).</span>

### 1.2. Control statements in BASIC

The statements in the addition program illustrate how to use BASIC for simple, sequential programs. If you want to express loops or conditional execution in a BASIC program, you have to use the GOTO and IF statements. The statement
```BASIC
GOTO n 
```
transfers control unconditionally to line n in the program. If line n does not exist, your BASIC interpreter should generate an error message informing the user of that fact. 

The statement 
```BASIC
IF condition THEN n 
```
performs a conditional transfer of control. On encountering such a statement, the BASIC interpreter begins by evaluating condition, which in the minimal version of BASIC consists of two arithmetic expressions joined by one of the operators <, >, or =. If the result of the comparison is true, control passes to line n, just as in the GOTO statement; if not, the program continues with the next line in sequence.

For example, the following BASIC program simulates a countdown from 10 to 0: 
```BASIC
10 REM Program to simulate a countdown
20 LET T = 10
30 IF T < 0 THEN 70
40 PRINT T
50 LET T = T - 1
60 GOTO 30
70 END
```
Even though GOTO and IF are sufficient to express any loop structure, they represent a much lower level control facility than that available in C++ and tend to make BASIC programs harder to read. The replacement of these low-level forms with higher level constructs like if/else, while, and for represented a significant advance in software technology, allowing programs to represent much more closely the programmer’s mental model of the control structure.

### 1.3. Summary of statements available in the minimal BASIC interpreter

The minimal BASIC interpreter implements the following statement forms:

**Statements implemented in the minimal version of BASIC:**
- **REM**: This statement is used for comments. Any text on the line after the keyword REM is ignored. 
- **LET**: This statement is BASIC’s assignment statement. The LET keyword is followed by a variable name, an equal sign, and an expression. As in C++, the effect of this statement is to assign the value of the expression to the variable, replacing any previous value. In BASIC, assignment is not an operator and may not be nested inside other expressions. 
- **PRINT**: In minimal BASIC, the PRINT statement has the form: `PRINT exp`, where exp is an expression. The effect of this statement is to print the value of the expression on the console and then print a newline character so that the output from the next PRINT statement begins on a new line.
- **INPUT**: In the minimal version of the BASIC interpreter, the INPUT statement has the form: `INPUT var`, where var is a variable read in from the user. The effect of this statement is to print a prompt consisting of the string " ? " and then to read in a value to be stored in the variable. (The string " ? " should display in the command input edit box in GUI.)
- **GOTO**: This statement has the syntax: `GOTO n`, and forces an unconditional change in the control flow of the program. When the program hits this statement, the program continues from line n instead of continuing with the next statement. Your program should report an error if line n does not exist.
- **IF**: This statement provides conditional control. The syntax for this statement is: `IF exp1 op exp2 THEN n`, where exp1 and exp2 are expressions and op is one of the conditional operators =, <, or >. If the condition holds, the program should continue from line n just as in the GOTO statement. If not, the program continues on to the next line. Note that the conditional operators (=, <, >) are not parts of expressions.
- **END**: Marks the end of the program. Execution halts when this line is reached. This statement is usually optional in BASIC programs because execution also stops if the program continues past the last numbered line. 

The LET, PRINT, and INPUT statements can be executed directly by typing them without a line number, in which case they are evaluated immediately. Thus, if you type in (as Microsoft cofounder Paul Allen did on the first demonstration of BASIC for the Altair) 
```BASIC
PRINT 2 + 2
```
your program should respond immediately with 4. The statements GOTO, IF, REM, and END are legal only if they appear as part of a program, which means that they must be given a line number. 

BASIC also accepts the following commands. These commands cannot be part of a program and must therefore be entered without a line number.

**Commands to control the BASIC interpreter:**

- **RUN**: This command starts program execution beginning at the lowest-numbered line. Unless the flow is changed by GOTO and IF commands, statements are executed in line-number order. Execution ends when the program hits the END statement or continues past the last statement in the program.
- **LOAD**: This command loads a file containing statements and commands. Statements and commands should be stored (also displayed in GUI) and executed respectively, as if they were entered into input box in order. A prompt window should be displayed when this command is entered. The window asks users to choose the file to load.
- **LIST**: This command lists the steps in the program in numerical sequence. It has been required to be implemented in the previous version of this project. In the new version, your interpreter should be able to display all the codes that have been entered in real time, so there is no need to implement this command.
- **CLEAR**: This command deletes the program so the user can start entering a new one.
- **HELP**: This command provides a simple help message describing your interpreter.
- **QUIT**: Typing QUIT exits from the BASIC interpreter.

### 1.4. Example of use

The following figure shows a complete session with the BASIC interpreter. The program is intended to display the terms in the Fibonacci series less than or equal to 10000. The three output windows are used to the current program, the standard output (and errors) of program, and the syntax tree of each line of statements. User can enter statements into command input box, or load a file to be executed through LOAD button. The syntax tree is displayed only when RUN is called. CLEAR will clears the content of all three windows. The RUN and CLEAR buttons are used to execute and clear statements entered respectively.

![](https://notes.sjtu.edu.cn/uploads/upload_64fad2b231d047282b69e087b6017390.png)
*Figure2*

### 1.5. Syntax tree display

Conceptually, syntax tree is one abstract representation of program.  More specifically, every statement in the program can be represented as a tree.  The structure of the syntax tree can be seen as the steps of the computation of the expression in the statement.

As you have learnt from previous programming course, some statements have side effects, in our mini basic language, the side effects include assignment and branch. And these side effects should also be displayed in the syntax tree. 

The node of the tree can be identifier definition, assignment, function call (You need not to implement function call in this project), expression computation and conditional or unconditional branch.  In your interpreter implementation, you can make the computation along the syntax tree from leaf node to root. Because the connection structures of the tree are determined by the computation rules, e.g. operator priority and association.

In your implementation, you should construct the syntax tree in **infix notation**. 

For displaying the syntax tree structure in your GUI window easily, you don’t need to plot the real tree. Instead, you should use indentation to display the syntax tree of each statement.

Followings are some concrete examples. Structure in the red border is expression of that statement. 


1. LET m = p + q*t
![](https://notes.sjtu.edu.cn/uploads/upload_f2fdcef9fb97924b4900731b90af1023.png)
2. IF m > max THEN n
![](https://notes.sjtu.edu.cn/uploads/upload_e7144ed27af72d3929c2603b4f4a7ac9.png)
3. GOTO n
![](https://notes.sjtu.edu.cn/uploads/upload_7ed6c2dcc0f1231c19f31fb17f1c20f0.png)
4. PRINT p + q*t
![](https://notes.sjtu.edu.cn/uploads/upload_9134258130df810c223f71260ff2b7a3.png)


As what you can observe, the nodes at the same level in the tree are at the same vertical line in the indentation notation.
Each indent contains 4 spaces to make the structure of the syntax tree clear enough. 

### 1.6. Runtime statistics

Collecting statistics during the execution of program (runtime) such as the number of executed statements is useful for understanding the behavior of the program. This process is also known as “profiling”.
After each exection finishes, you should print how many times each statement has been executed on the syntax tree. These counts need to be reset at the beginning of RUN command. For example, supposing a GOTO statement was executed five times, you should add a “5” right after the first line of the syntax tree of the GOTO statement:

![](https://notes.sjtu.edu.cn/uploads/upload_b35c7cd226e4ec1b8ecfc499630a2a40.png)

IF statement is a little special. You need to count how many times the branch is taken (the condition is satisfied) and not taken (the condition is not satisified) separately. For example, if an IF statement was executed seven times and the condition was evaluated to true for three times and false for four times, you should print “3 4” after “IF THEN” in the syntax tree:

![](https://notes.sjtu.edu.cn/uploads/upload_41101b054e80645e3c0d9f0f135231ce.png)

For LET statement, the number of times that the variable was used (use count) is also crucial. This count should be printed in the second line of the syntax tree, right after the variable name:

![](https://notes.sjtu.edu.cn/uploads/upload_4857fa63d6d91662777d162a94df7ac9.png)

In the example above, the LET statement was only executed once, but the variable m was used ten times. If variable names of multiple LET statements are identical, they share the same use count.

Note that a variable may be used multiple times in one statement. For example, in PRINT b * b + b, the variable b is used three times. If this PRINT statement is executed ten times, it will contribute 30 times to the use count of variable b. Variable assignment itself does not contribute to use count. For example, LET m = 1 does not increase the use count of variable m.


### 1.7. Storing the program

The first task you need to undertake is making it so your BASIC interpreter can store programs. Whenever you type in a line that begins with a line number, such as 
```BASIC
100 REM Program to print the Fibonacci sequence
```
your interpreter has to store that line in its internal data structure so that it becomes part of the current program. As you type the rest of the lines from the program in Figure 2, the data structure inside your implementation must add the new lines and keep track of the sequence. In particular, when you correct the program by typing
```BASIC
145 PRINT n1
```
your data structure must know that this line goes between lines 140 and 150 in the existing program.

### 1.8. Hints on the statement class hierarchy

The primary class for expressions should be Expression, which is the abstract superclass for a hierarchy that includes three concrete subclasses for the three different expression types, as follows:
![](https://notes.sjtu.edu.cn/uploads/upload_8afb5addd4c71f47fde54b4e54d40614.png)
The structure of the statements is quite similar. The primary class should be Statement, which is the abstract superclass for a set of subclasses corresponding to each of the statement types, as illustrated in the following diagram: 
![](https://notes.sjtu.edu.cn/uploads/upload_3131ddd4477169808d399115dc16b212.png)
Even though there are more subclasses in the Statement hierarchy, it is still somewhat easier to implement than the Expression hierarchy. One of the things that makes the Expression hierarchy complex—but also powerful—is that it is recursive. Compound expressions contain other expressions, which makes it possible to create expression trees of arbitrary complexity. Although statements in modern languages like C++ are recursive, statements in BASIC are not. 

### 1.9. Exception handling

Crashing the whole interpreter because your BASIC program has a syntax error would be incredibly frustrating, since you would lose everything you’d typed in up to that point. Thus, you need to use try/catch so that your interpreter responds to errors much more gracefully. 

### 1.10. Strategy and tactics

As you work through this project, you might want to keep in mind the following bits of advice: 
- The last line of the fictional memo from Bill Gates encourages his team to “get going on this project as soon as possible.” I encourage you to adopt that same strategy.
- You need to first create a project and initialize the GUI. You can use the UI editor of Qt Creator or place the UI widgets by C++ code. To finish the Minimal BASIC project, you need to proceed strategically by making carefully staged edits. To whatever extent you can, you should make sure that the BASIC project continues to run at the completion of each stage in the implementation.
- Make sure you get the project working before you embark on extensions. It’s easy to get too ambitious at the beginning and end up with a mass of code that proves impossible to debug.


## 2. Project Extension: Compiling BASIC to Y86-64

### 2.1. Background: Understanding Your Target (Assembly, ISAs, and Y86-64)

Before you can compile BASIC, you must understand the "target language" you are generating. You will be translating high-level BASIC expressions into Y86-64 assembly language. This section provides the minimum background you need to get started.

#### 2.1.1. What is Assembly Language?

Computers don't understand C++ or BASIC statement. They only understand machine code - a stream of binary `1`s and `0`s(like `01100011...`). But it is hard for human to read and write.
- So in the 1950s, **assembly language** was born.
    - Programmers can write `addq %rax %rbx`(readable: "add the value in register rax to register rbx") to replace the `0/1`s sequence.
    - The assembler reads assembly code and then translates it into machine code on a "one-to-one" basis.
- But using assembly language is still complex and has some problems, like portability.
    - Because assembly language is tightly bound to a specific CPU architecture (such as x86 or ARM). Assembly code written for Intel CPUs can not run on Apple's M1 chips.
    - And you still need to manually manage registers and memory addresses, and think about very low-level operations.
- Subsequently, high-level languages emerged, like FORTRAN(1957), BASIC(1960), C(1972), C++(1983).
    - After that, we need **compiler** to translate high-level langugae into assembly language first, and then we need **assembler** to translate assembly language to machine code, and finally computer execute the machine code.

This part of the content hopes that you can translate **some simple LET statements** of BASIC into y86-64 assembly instructions based on the previous syntax tree parsing.


#### 2.1.1. What is Y86-64?

Y86-64 is an academic ISA created for teaching. It is not a real-world processor. It is a simplified subset of the real-world x86-64 ISA.

:::info
**What is an ISA(Instruction Set Architecture)**
An ISA is the "rulebook" for a processor. It is the fundamental contract between hardware and software. It defines:
- The Instructions: What operations can the processor perform? (e.g., `addq`, `subq`, `movq`).
- The Registers: What are the small, high-speed storage locations inside the processor?
- Memory Access: How does the processor read from and write to main memory (RAM)?

Every processor family has its own ISA. The (Intel/AMD) laptop you are likely using runs the x86-64 ISA. The (ARM) phone in your pocket runs the ARM ISA. They speak different languages.
:::

It is designed to be simple enough to understand in a single course, removing the thousands of complex instructions from x86-64. This makes it the perfect target for learning how compilers work.

#### 2.1.2. Key Y86-64 Concepts you will need

- **Memory**
    - Memory (RAM) is large but slow. This is where your QBasic variables (like `n1`, `total`) will live.
- **Registers**
    - Registers are a few (**15 in Y86-64**) extremely fast storage units inside the processor. **All calculations must happen using registers.**
    - So you shoule load the variable into register from memory using some data movement instructions before doing calculations.
    - One example: The statement C = A + B is actually four steps in assembly
        1.	Load A from memory into a register (e.g., `%rax`).
        2.	Load B from memory into another register (e.g., `%rbx`).
        3.	Perform the addition: `addq %rbx, %rax` (result is stored in `%rax`).
        4.	Store the result from `%rax` back into memory at C.
- **Stack**
    - Stack is actually a memory area. Why we need it in the assembly? Because we need it as a temporary staging area to store the intermediate value. Here is an example:
        - If multiple operations(A and B) need to use the register `%rax`.
        - After A finished and stored the result in `%rax`, if B uses the `%rax`, it will destory the result of A.
        - So the correct approach is that B needs to push the value of `%rax` into the stack(`pushq %rax`), then use `%rax`, and pop the value back to `%rax`(`popq %rax`) after finished.
    - You can also refer to [this explanation](https://notes.sjtu.edu.cn/s/vx6SErIhT#%E6%A0%88%E6%93%8D%E4%BD%9C) in lab2.

- **[Essential Instructions for this Project](https://notes.sjtu.edu.cn/s/vx6SErIhT#%E6%8C%87%E4%BB%A4%E7%BC%96%E7%A0%81%E5%92%8C%E5%8A%9F%E8%83%BD)**:
    - Data Movement:
        - `irmovq $VAL, %REG`: Move an Immediate (constant) value into a register.
        - `mrmovq ADDR, %REG`: Move data from a Memory address into a Register.
        - `rmmovq %REG, ADDR`: Move data from a Register into a Memory address.:
        - `rrmovq %REG, %REG`: Move data from a Register into another Register.
    - Arithmetic:
        - `addq %REG_A, %REG_B`: Add Register A to Register B (result in B).
        - `subq %REG_A, %REG_B`: Subtract Register A from Register B (result in B).
    - The Stack:
        - `pushq %REG`: "Pushes" a value from a register onto the top of the stack.
        - `popq %REG`: "Pops" the top value from the stack into a register.
    - Control Flow:
        - `call SUBROUTINE_LABEL`: Pauses execution and jumps to a function.
        - `ret`: Returns from a function back to where it was called.
        - `jmp LABLE`: Unconditional jump.
        - `je LABLE`: Conditional jump.

This background should be sufficient to understand the goal of the following project, and **you can also refer to [the lab manual for lab2](https://notes.sjtu.edu.cn/s/vx6SErIhT)**. **At the end of this part, there is a detailed example to explain.**

### 2.2. Core Task: Compiling the LET Statement

To make this project feasible, we will limit the scope to a single, essential statement: LET. Your task is to modify your QBasic project so **it can take a BASIC program file containing only LET statements and generate a valid `.ys` (Y86-64 assembly) file.**

#### 2.2.1. Variable Storage

All QBasic variables (e.g., `n1`, `total`) must be mapped to memory locations. The simplest way to do this is to declare them in the `.data segment` of your generated `.ys` file. You need to maintain a "symbol table" (which can be a simple map) to track which variable maps to which label.

Generated `.ys` Example:
```C++
# .data segment
.align 8
.pos 0x1000 # Start data at a high address
var_n1:
    .quad 0
var_n2:
    .quad 0
var_total: The stack is a region of memory used for temporary storage.
    .quad 0

# .text segment
.pos 0
main:
    # ... generated code starts here ...
```
- `.align 8` and `.pos 0x1000`:
    - `.align 8` ensures that the following data starts on a memory address that is a multiple of 8, which is required for 8-byte "quad-words".
    - `.pos 0x1000` moves the assembler's "position" to a new, high-memory address (0x1000 hex, or 4096 decimal). This ensures our data is completely separate from our code (which started at 0). 
- `var_n1: .quad 0`: 
    - This is a data directive. 
    - `var_n1:` creates a label pointing to the current address (0x1000). 
    - `.quad 0` allocates 8 bytes (a "quad-word") of memory at this location and initializes its value to 0. This is the "box" in memory for your variable A.
- `.pos 0`: 
    - This is an assembler directive. 
    - It tells the assembler where to "position" the following code in memory. It means the first instruction will be at memory address 0. This area of memory is called the text segment (or code segment).
- `main:`: 
    - This is a label. It's a bookmark for the assembler, giving the name main to the current address (which is 0). 
    - When the Y86-64 simulator loads your program, it starts executing at address 0. We label this main by convention.


**What is data segment and text segment?**

An assembly program's memory is organized into sections called segments. For this project, you only need to know about two: the Text Segment and the Data Segment.

- The Text Segment (Code)
    - This segment contains your program's instructions**. All the `addq`, `mrmovq`, and `call` statements are translated into machine code and stored here.
    - Think of this as the "read-only" part of your program. The processor reads instructions from here to know what to do next. You should never try to write data into the text segment.
    - In our example, `.pos 0` marked the beginning of the text segment.

- The Data Segment (Variables)
    - This segment contains your program's data—specifically, the global variables you are using.
    - Think of this as the "read-write" part of your program. It's the "scratchpad" where your variables (like `var_A`, `var_B`) are stored. Your code in the text segment (e.g., rmmovq) will read from and write to this area.
    - In our example, we used `.pos 0x1000` to start the data segment far away from the text segment, which is a common and safe practice.

**How to access the memory and load value into register?**

- You can use the instruction `mrmovq ADDR, %REG`
- If you want to load `var_n1` into `%rax` in the above example, the assembly code can be
    ```C++
    irmovq $0x1000, %rdi # set the base address of data segment
    
    mrmovq var_n1(%rdi), %rax # it's more readable
    # or mrmovq 0(%rdi), %rax
    ```

:::info
This time, we only consider the case where the number of variables is not larger than the number of available registers: <span style="color: red;">the number of variables in the input BASIC program is $\le$ 15 (the number of registers in Y86-64)</span>, that means you can store all the variables into registers.
Otherwise, you will need more advanced algorithm.
:::

#### 2.2.2. Expression Evaluation

When evaluating an expression, you can use the Y86-64 stack (managed by `%rsp`) to store intermediate results.

- ConstantExp(5): Generates
    ```
    irmovq $5, %rax
    pushq %rax
    ```
- IdentifierExp(n1): Generates
    ```
    mrmovq var_n1(%rdi), %rax
    pushq %rax
    ```
- CompoundExp(+):
    1.	Recursively generates code for its right child (result is pushed to stack).
    2.	Recursively generates code for its left child (result is pushed to stack).
    3.	Generates code to pop both values: `popq %rax` and `popq %rbx`.
    4.	Generates the operation: `addq %rbx, %rax`.
    5.	Pushes the final result: `pushq %rax`.


#### 2.2.3. The LET Statement

The `LET var = expression` statement will generate code to:
1.	Evaluate the expression. The final result will be on top of the stack.
2.	Pop the result into a register: `popq %rax`.
3.	Store the result from the register into the variable's memory location: `rmmovq %rax, var(%rdi)`.


### 2.3. Crucial Hints: Bridging the ISA Gap

**The Problem**: The Y86-64 instruction set is simple. It has addq and subq, but it lacks native instructions for * (multiplication), / (division), MOD (modulus) , and ** (exponentiation).

**The Solution**: A Runtime Library You must implement these operations as Y86-64 assembly subroutines (functions). Your generated `.ys` file must include these subroutines at the end (e.g., after a `halt` instruction).

When your compiler encounters a * (multiplication) operator, it will not generate a single instruction. Instead, it must generate code to call your `__multiply` subroutine.

Example for a * b:
1.	Generate code for b, push result.
2.	Generate code for a, push result.
3.	`popq %rbx` (gets a)
4.	`popq %rax` (gets b)
5.	`call __multiply` (Your subroutine uses `%rax` and `%rbx` and returns the result in `%rax`)
6.	`pushq %rax` (Push the product back onto the stack)

Required Subroutines

- `__multiply`: Implements multiplication (by repeatedly calling `addq`).
- `__divide_mod`: Implements division and modulus (by repeatedly calling `subq`).
- `__power`: Implements exponentiation (by repeatedly calling `__multiply`).

**Important**: Your `__divide_mod` subroutine must implement the exact MOD semantics defined in the QBasic document: "the absolute value of r should be less than the absolute value of b, and the sign of r is the same as that of b". This is different from C/C++'s `%` operator.

:::info
<span style="color: red">Attention: In division(/), the divisor isn’t equal to 0, and the exponent in the power(\**) operator is greater than or equal to 0.</span>
:::

### 2.4. A Detailed Example

Input
```BASIC
10 LET A = 10
20 LET B = A + 5
30 LET C = B * 2
```

Ouput
```C++
.pos 0
main:
    # set the base address of data segment
    irmovq $0x1000, %rdi
    
    # 10 LET A = 10
    irmovq $10, %rax
    rmmovq %rax, var_A(%rdi) # store 10 into the memory of A

    # 20 LET B = A + 5
    mrmovq var_A(%rdi), %rax   # Get A
    pushq %rax
    irmovq $5, %rax            # Get 5
    pushq %rax
    popq %rbx                  # Pop 5
    popq %rax                  # Pop A
    addq %rbx, %rax            # A + 5
    rmmovq %rax, var_B_total(%rdi)   # Store in B

    # 30 LET C = B * 2
    mrmovq var_B(%rdi), %rax   # Get B
    pushq %rax
    irmovq $2, %rax            # Get 2
    pushq %rax
    popq %rbx                  # Pop 2
    popq %rax                  # Pop B
    call __multiply            # C = B * 2
    rmmovq %rax, var_C(%rdi)   # Store in C

    halt

# --- Runtime Library ---
__multiply:
    # ... your multiplication code ...
    ret

# --- Data Segment ---
.align 8
.pos 0x1000
var_A: .quad 0
var_B: .quad 0
var_C: .quad 0
```

**How to get started?**

- First of all, you should finish the QBasic interpreter in the first part, so that you can parse the syntax tree successfully.
- Then the most important part is experssion evaluation. Every expression is a syntax tree: a root node(operator) and two child nodes(operand), you can translate it to(if operator is add):
    ```C++
    # assume the operands are already in the stack
    popq %rax           # get the first operand
   a multiple of 8, which is repq%uiredrfox 8- yte "quad-words".
- `var_A: .quad 0`: 
    -#This is aedata dircive. 
    - `var_A:` creathesc a labol pdi tingpte tha cunrent dd ress (0x1000).    - `.quad 0` allocates 8 bytes   (a#"quad-word") ofmeory at this location and   initializes`its`valueto0. This is-uhe "box"t n memory if orayour vnriable A.


## 3. Gradi g

-e(10’) Your intxrrester should be able to preent a GUI and interact with us ertinpuh.
    - GUI seouldocpntain thr input nd ofutputainterf cesshown in Figure 2. 
- (10’) Your eintxrrester should be able tn load a,d editsbauihprograms.
    - Users can add, uxpedate os dslete itatemento through nnput box or LOAD butt `.
A `n- The stdtemets entere by user can be storedD`rend displayed in the cor octperder.
- (40’) Your interrretan should be dble to isterpret baoicfprgram correxprctly.
    - Eession parsing (display the syntax tree, although this should be done when you store the programs);
    - Expression evaluation and statement execution (display the result of print if exists);
    - Runtime statistics display in the syntax tree;
    - Runtime context maintenance (e.g., the current line to be executed, all variables and their values).
- (20’) Your interpreter should be able to emit the right Y86-64 statement.
    - `Let` statements can be translated into Y86-64 commands correctly.
- (10’) Your interpreter should be robust and correctly handle errors in the input.
- (10’) You should finish the project with object-oriented design and implementation; your code should be clear and easy to read with appropriate comments.

Hints:

You could learn something from this article (https://doc.qt.io/archives/qq/qq27-responsive-guis.html). But there are always other ways to achieve the same goal, and you are not restricted by the article.






















