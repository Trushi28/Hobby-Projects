🔬 Compilers & Language Design

    An exploration into the art of language creation, from parsing and semantic analysis to code generation.

Welcome to the Compilers section of my hobby projects! This folder is dedicated to my passion for understanding how programming languages work under the hood. Here, I design and build my own languages from scratch, experimenting with unique features, parsing techniques, and compiler architectures.

These projects are my practical deep-dive into topics like lexical analysis, abstract syntax trees, semantic analysis, and code generation.
🚀 Project Showcase

<table>
<tr>
<td align="center" width="50%">
1. Flux (in Rust 🦀)

A Modern Pragmatic Language

A robust, multi-stage compiler targeting LLVM IR, designed for safety and expressiveness with a clean, modern syntax.

🔥 Core Concepts:

    Type-Locking Dynamics

    Pragma-Controlled Syntax

    Temporal (Time-Aware) Variables

    Functional Pipeline Operations

</td>
<td align="center" width="50%">
2. ZenLang (in C ⚙️)

An Experimental Function-Oriented Language

A highly experimental compiler exploring unconventional features like custom memory management and advanced functional programming concepts.

🔥 Core Concepts:

    Immutable by Default

    Custom Memory Zones

    Automatic Function Currying

    Advanced Pattern Matching

</td>
</tr>
</table>
1. Project Deep Dive: Flux (Rust 🦀)

    A modern systems language that blends the safety of Rust with the flexibility of dynamic typing.

✨ Key Features & Philosophy

Flux is designed to be both powerful and developer-friendly. Its core philosophy is to provide high-level, expressive features while maintaining the performance and safety expected from a compiled language.

Feature
	

Description
	

Strategic Advantage

Type-Locking
	

A variable's type is set on first assignment and cannot be changed.
	

Prevents a common class of runtime errors while avoiding verbose type declarations.

Temporal Variables
	

Access a variable's historical values (e.g., x[timestamp]).
	

Enables powerful patterns for state tracking, debugging, and data analysis.

**Pipeline Operator `
	

`**
	

Chain functions together in a clean, readable sequence.

Syntax Pragmas
	

Switch between {} braces and indentation with #pragma.
	

Gives developers control over the coding style that suits them best.
🏗️ Architectural Overview

Flux is built with a classic, robust compiler architecture that ensures scalability and correctness.

graph TD
    A[Source Code (.flux)] --> B{Lexer};
    B --> C[Tokens];
    C --> D{Parser};
    D --> E[Abstract Syntax Tree (AST)];
    E --> F{Semantic Analyzer};
    F --> G[Verified AST];
    G --> H{Code Generator};
    H --> I[LLVM IR];

📈 Current Status: Work-in-Progress

    ✅ What Works: Parsing of core syntax (let, if, func), AST generation, and valid LLVM IR output for basic arithmetic.

    🟡 What's Next: Implementing a full runtime type system beyond just numbers, completing the parser logic for temporal access and re-assignment, and building a proper scoping system.

🚀 Future Roadmap

    [ ] Implement a Runtime Type System: Introduce a Value enum to handle strings, booleans, and objects.

    [ ] Complete Parser Logic: Add rules for statement-level assignments and match expressions.

    [ ] Fix Indentation-Based Parsing: Have the lexer produce Indent/Dedent tokens.

    [ ] Build a Standard Library: Integrate a set of built-in functions.

    [ ] Full Temporal Variable Implementation: Connect the AST to a runtime lookup function.

🛠️ How to Compile & Run

    Navigate to the project directory: cd path/to/FluxCompiler/

    Create a source file (e.g., test.flux):

    // test.flux
    let result = (10 + 20) * 2
    print(result)

    Compile and run using Cargo:

    cargo run -- test.flux

2. Project Deep Dive: ZenLang (C ⚙️)

    An exploration of advanced, unconventional language features implemented in pure C.

✨ Key Features & Philosophy

ZenLang is an academic and experimental project focused on "what if?" scenarios in language design. It prioritizes developer control and advanced functional concepts over simplicity.

Feature
	

Description
	

Strategic Advantage

Memory Zones
	

Allocate variables in named memory pools for custom management.
	

Offers fine-grained control over memory layout and performance, similar to data-oriented design.

Auto-Currying
	

Partially applied functions automatically return new functions.
	

Enables elegant and powerful functional programming patterns.

Immutable by Default
	

All variables are constants unless explicitly stated otherwise.
	

Enforces a safer, more predictable state management model.

Pragma Toggles
	

Enable/disable core features like pattern matching via #pragma.
	

Allows the language itself to be customized for different use cases.
🏗️ Architectural Overview

ZenLang is in an early conceptual stage. Its current architecture is a hybrid model that needs refactoring.

    Current Model: A linear Interpreter/Compiler. It parses and executes declarations immediately, then dumps the final memory state to assembly.

    Required Refactor: The architecture must be updated to use an Abstract Syntax Tree (AST). This is the critical next step to enable the compilation of actual program logic.

📈 Current Status: Conceptual / Proof-of-Concept

    ✅ What Works: The lexer correctly tokenizes all custom syntax. The program can parse simple declarations and generate a basic assembly file representing the final variable values.

    🔴 Core Pending Task: The compiler lacks an AST. Without it, no program logic (expressions, control flow, function calls) can be compiled. This is the highest-priority task.

🚀 Future Roadmap

    [ ] Introduce an AST: Refactor the parser to build a tree structure instead of interpreting code directly.

    [ ] Build a Real Code Generator: Create a new generator that "walks" the AST to produce assembly.

    [ ] Implement Function Bodies: Parse and store the code inside functions in the AST.

    [ ] Develop Core Features: Implement the actual logic for Memory Zones, Auto-Currying, and Pattern Matching.

    [ ] Create a Type System: Formalize the handling of numbers, strings, and objects.

🛠️ How to Compile & Run

    Compile the C source file:

    gcc -o zen_compiler zenlang.c

    Run the compiler:
    The compiler will automatically create an example.zen file if none is provided.

    ./zen_compiler example.zen


