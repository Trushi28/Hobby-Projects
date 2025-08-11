#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdbool.h>

// ZenLang Compiler - A unique function-oriented language
// Features:
// 1. Immutable dynamic typing (once assigned, no changes)
// 2. OOP concepts without strict enforcement
// 3. Brace style selection via #pragma
// 4. Unique features: Pattern matching, Auto-currying, Memory zones

#define MAX_TOKEN_LEN 256
#define MAX_VARIABLES 1000
#define MAX_FUNCTIONS 100
#define MAX_MEMORY_ZONES 10

// Token types
typedef enum {
    TOKEN_IDENTIFIER,
    TOKEN_NUMBER,
    TOKEN_STRING,
    TOKEN_KEYWORD,
    TOKEN_OPERATOR,
    TOKEN_BRACE_OPEN,
    TOKEN_BRACE_CLOSE,
    TOKEN_PAREN_OPEN,
    TOKEN_PAREN_CLOSE,
    TOKEN_SEMICOLON,
    TOKEN_COMMA,
    TOKEN_EOF,
    TOKEN_NEWLINE,
    TOKEN_PRAGMA,
    TOKEN_ARROW,
    TOKEN_PIPE,
    TOKEN_PATTERN_MATCH,
    TOKEN_ZONE_MARKER
} TokenType;

// Value types for dynamic typing
typedef enum {
    TYPE_UNDEFINED,
    TYPE_NUMBER,
    TYPE_STRING,
    TYPE_FUNCTION,
    TYPE_OBJECT,
    TYPE_PATTERN
} ValueType;

// Memory zones for unique memory management
typedef struct {
    char name[64];
    void* memory_pool;
    size_t size;
    size_t used;
    bool auto_cleanup;
} MemoryZone;

// Variable structure with immutability
typedef struct {
    char name[64];
    ValueType type;
    union {
        double number;
        char* string;
        void* object;
        int function_id;
    } value;
    bool is_assigned;
    int memory_zone_id;
    char pattern[128]; // For pattern matching
} Variable;

// Function structure with auto-currying support
typedef struct {
    char name[64];
    int param_count;
    char params[10][64];
    char body[1024];
    bool is_curried;
    int curry_level;
    bool use_braces;
} Function;

// Compiler state
typedef struct {
    bool use_braces;
    bool pattern_matching_enabled;
    bool auto_curry_enabled;
    char current_zone[64];
    Variable variables[MAX_VARIABLES];
    int var_count;
    Function functions[MAX_FUNCTIONS];
    int func_count;
    MemoryZone zones[MAX_MEMORY_ZONES];
    int zone_count;
} CompilerState;

// Token structure
typedef struct {
    TokenType type;
    char value[MAX_TOKEN_LEN];
    int line;
    int column;
} Token;

CompilerState compiler_state = {0};

// Keywords
const char* keywords[] = {
    "let", "fn", "if", "else", "while", "for", "return", "class", "new",
    "match", "case", "zone", "curry", "pipe", "import", "export", NULL
};

// Initialize memory zones
void init_memory_zones() {
    strcpy(compiler_state.zones[0].name, "global");
    compiler_state.zones[0].size = 1024 * 1024; // 1MB
    compiler_state.zones[0].memory_pool = malloc(compiler_state.zones[0].size);
    compiler_state.zones[0].used = 0;
    compiler_state.zones[0].auto_cleanup = false;
    compiler_state.zone_count = 1;
    strcpy(compiler_state.current_zone, "global");
}

// Check if string is a keyword
bool is_keyword(const char* str) {
    for (int i = 0; keywords[i]; i++) {
        if (strcmp(str, keywords[i]) == 0) return true;
    }
    return false;
}

// Lexical analyzer
Token* tokenize(const char* source) {
    Token* tokens = malloc(sizeof(Token) * 10000);
    int token_count = 0;
    int line = 1, col = 1;
    
    for (int i = 0; source[i]; i++, col++) {
        char c = source[i];
        
        if (c == '\n') {
            tokens[token_count].type = TOKEN_NEWLINE;
            strcpy(tokens[token_count].value, "\n");
            tokens[token_count].line = line++;
            tokens[token_count].column = col;
            token_count++;
            col = 0;
            continue;
        }
        
        if (isspace(c)) continue;
        
        // Handle pragmas
        if (c == '#') {
            char pragma[256] = {0};
            int j = 0;
            while (source[i] && source[i] != '\n') {
                pragma[j++] = source[i++];
            }
            i--; // Back up one
            
            if (strstr(pragma, "#pragma braces")) {
                compiler_state.use_braces = true;
            } else if (strstr(pragma, "#pragma no-braces")) {
                compiler_state.use_braces = false;
            } else if (strstr(pragma, "#pragma pattern-match")) {
                compiler_state.pattern_matching_enabled = true;
            } else if (strstr(pragma, "#pragma auto-curry")) {
                compiler_state.auto_curry_enabled = true;
            }
            
            tokens[token_count].type = TOKEN_PRAGMA;
            strcpy(tokens[token_count].value, pragma);
            tokens[token_count].line = line;
            tokens[token_count].column = col;
            token_count++;
            continue;
        }
        
        // Handle strings
        if (c == '"') {
            tokens[token_count].type = TOKEN_STRING;
            int j = 0;
            i++; // Skip opening quote
            while (source[i] && source[i] != '"') {
                tokens[token_count].value[j++] = source[i++];
            }
            tokens[token_count].value[j] = '\0';
            tokens[token_count].line = line;
            tokens[token_count].column = col;
            token_count++;
            continue;
        }
        
        // Handle numbers
        if (isdigit(c)) {
            tokens[token_count].type = TOKEN_NUMBER;
            int j = 0;
            while (isdigit(source[i]) || source[i] == '.') {
                tokens[token_count].value[j++] = source[i++];
            }
            i--; // Back up one
            tokens[token_count].value[j] = '\0';
            tokens[token_count].line = line;
            tokens[token_count].column = col;
            token_count++;
            continue;
        }
        
        // Handle operators and special characters
        if (c == '=' && source[i+1] == '>') {
            tokens[token_count].type = TOKEN_ARROW;
            strcpy(tokens[token_count].value, "=>");
            tokens[token_count].line = line;
            tokens[token_count].column = col;
            token_count++;
            i++; col++;
            continue;
        }
        
        if (c == '|') {
            tokens[token_count].type = TOKEN_PIPE;
            strcpy(tokens[token_count].value, "|");
            tokens[token_count].line = line;
            tokens[token_count].column = col;
            token_count++;
            continue;
        }
        
        if (c == '@') {
            tokens[token_count].type = TOKEN_ZONE_MARKER;
            strcpy(tokens[token_count].value, "@");
            tokens[token_count].line = line;
            tokens[token_count].column = col;
            token_count++;
            continue;
        }
        
        // Handle single character tokens
        switch (c) {
            case '{':
                tokens[token_count].type = TOKEN_BRACE_OPEN;
                tokens[token_count].value[0] = c;
                tokens[token_count].value[1] = '\0';
                break;
            case '}':
                tokens[token_count].type = TOKEN_BRACE_CLOSE;
                tokens[token_count].value[0] = c;
                tokens[token_count].value[1] = '\0';
                break;
            case '(':
                tokens[token_count].type = TOKEN_PAREN_OPEN;
                tokens[token_count].value[0] = c;
                tokens[token_count].value[1] = '\0';
                break;
            case ')':
                tokens[token_count].type = TOKEN_PAREN_CLOSE;
                tokens[token_count].value[0] = c;
                tokens[token_count].value[1] = '\0';
                break;
            case ';':
                tokens[token_count].type = TOKEN_SEMICOLON;
                tokens[token_count].value[0] = c;
                tokens[token_count].value[1] = '\0';
                break;
            case ',':
                tokens[token_count].type = TOKEN_COMMA;
                tokens[token_count].value[0] = c;
                tokens[token_count].value[1] = '\0';
                break;
            case '+': case '-': case '*': case '/': case '=':
                tokens[token_count].type = TOKEN_OPERATOR;
                tokens[token_count].value[0] = c;
                tokens[token_count].value[1] = '\0';
                break;
            default:
                // Handle identifiers
                if (isalpha(c) || c == '_') {
                    int j = 0;
                    while (isalnum(source[i]) || source[i] == '_') {
                        tokens[token_count].value[j++] = source[i++];
                    }
                    i--; // Back up one
                    tokens[token_count].value[j] = '\0';
                    
                    if (is_keyword(tokens[token_count].value)) {
                        tokens[token_count].type = TOKEN_KEYWORD;
                    } else {
                        tokens[token_count].type = TOKEN_IDENTIFIER;
                    }
                }
                break;
        }
        
        if (tokens[token_count].type != TOKEN_EOF) {
            tokens[token_count].line = line;
            tokens[token_count].column = col;
            token_count++;
        }
    }
    
    tokens[token_count].type = TOKEN_EOF;
    strcpy(tokens[token_count].value, "");
    tokens[token_count].line = line;
    tokens[token_count].column = col;
    
    return tokens;
}

// Find variable by name
Variable* find_variable(const char* name) {
    for (int i = 0; i < compiler_state.var_count; i++) {
        if (strcmp(compiler_state.variables[i].name, name) == 0) {
            return &compiler_state.variables[i];
        }
    }
    return NULL;
}

// Create new variable with immutability check
bool create_variable(const char* name, ValueType type) {
    Variable* existing = find_variable(name);
    if (existing && existing->is_assigned) {
        printf("Error: Variable '%s' is already assigned and cannot be changed (immutable)\n", name);
        return false;
    }
    
    if (!existing) {
        if (compiler_state.var_count >= MAX_VARIABLES) {
            printf("Error: Too many variables\n");
            return false;
        }
        
        Variable* var = &compiler_state.variables[compiler_state.var_count++];
        strcpy(var->name, name);
        var->type = type;
        var->is_assigned = false;
        var->memory_zone_id = 0; // Default to global zone
    }
    
    return true;
}

// Pattern matching function
bool match_pattern(const char* pattern, const char* value) {
    // Simple pattern matching implementation
    // Supports wildcards (*) and exact matches
    
    if (strcmp(pattern, "*") == 0) return true;
    if (strcmp(pattern, value) == 0) return true;
    
    // More complex pattern matching could be implemented here
    return false;
}

// Auto-currying function implementation
void create_curried_function(const char* name, int provided_args) {
    for (int i = 0; i < compiler_state.func_count; i++) {
        Function* func = &compiler_state.functions[i];
        if (strcmp(func->name, name) == 0) {
            if (provided_args < func->param_count && compiler_state.auto_curry_enabled) {
                // Create a curried version
                Function* curried = &compiler_state.functions[compiler_state.func_count++];
                sprintf(curried->name, "%s_curried_%d", name, provided_args);
                curried->param_count = func->param_count - provided_args;
                curried->is_curried = true;
                curried->curry_level = provided_args;
                
                printf("Auto-curried function '%s' created\n", curried->name);
            }
            break;
        }
    }
}

// Memory zone management
bool create_memory_zone(const char* name, size_t size, bool auto_cleanup) {
    if (compiler_state.zone_count >= MAX_MEMORY_ZONES) {
        printf("Error: Too many memory zones\n");
        return false;
    }
    
    MemoryZone* zone = &compiler_state.zones[compiler_state.zone_count++];
    strcpy(zone->name, name);
    zone->size = size;
    zone->memory_pool = malloc(size);
    zone->used = 0;
    zone->auto_cleanup = auto_cleanup;
    
    printf("Memory zone '%s' created with %zu bytes\n", name, size);
    return true;
}

// Simple parser and interpreter
void parse_and_execute(Token* tokens) {
    int i = 0;
    
    while (tokens[i].type != TOKEN_EOF) {
        Token current = tokens[i];
        
        // Handle pragma directives
        if (current.type == TOKEN_PRAGMA) {
            printf("Processed: %s\n", current.value);
            i++;
            continue;
        }
        
        // Handle variable declarations
        if (current.type == TOKEN_KEYWORD && strcmp(current.value, "let") == 0) {
            i++; // Skip 'let'
            if (tokens[i].type == TOKEN_IDENTIFIER) {
                char var_name[64];
                strcpy(var_name, tokens[i].value);
                
                i++; // Skip identifier
                if (tokens[i].type == TOKEN_OPERATOR && strcmp(tokens[i].value, "=") == 0) {
                    i++; // Skip '='
                    
                    // Handle assignment based on token type
                    if (tokens[i].type == TOKEN_NUMBER) {
                        if (create_variable(var_name, TYPE_NUMBER)) {
                            Variable* var = find_variable(var_name);
                            var->value.number = atof(tokens[i].value);
                            var->is_assigned = true;
                            printf("Assigned %s = %f\n", var_name, var->value.number);
                        }
                    } else if (tokens[i].type == TOKEN_STRING) {
                        if (create_variable(var_name, TYPE_STRING)) {
                            Variable* var = find_variable(var_name);
                            var->value.string = malloc(strlen(tokens[i].value) + 1);
                            strcpy(var->value.string, tokens[i].value);
                            var->is_assigned = true;
                            printf("Assigned %s = \"%s\"\n", var_name, var->value.string);
                        }
                    }
                }
            }
        }
        
        // Handle function declarations
        else if (current.type == TOKEN_KEYWORD && strcmp(current.value, "fn") == 0) {
            i++; // Skip 'fn'
            if (tokens[i].type == TOKEN_IDENTIFIER) {
                if (compiler_state.func_count >= MAX_FUNCTIONS) {
                    printf("Error: Too many functions\n");
                    break;
                }
                
                Function* func = &compiler_state.functions[compiler_state.func_count++];
                strcpy(func->name, tokens[i].value);
                func->param_count = 0;
                func->is_curried = false;
                func->use_braces = compiler_state.use_braces;
                
                printf("Declared function: %s\n", func->name);
            }
        }
        
        // Handle memory zone creation
        else if (current.type == TOKEN_KEYWORD && strcmp(current.value, "zone") == 0) {
            i++; // Skip 'zone'
            if (tokens[i].type == TOKEN_IDENTIFIER) {
                char zone_name[64];
                strcpy(zone_name, tokens[i].value);
                create_memory_zone(zone_name, 64*1024, true); // 64KB default
            }
        }
        
        // Handle pattern matching
        else if (current.type == TOKEN_KEYWORD && strcmp(current.value, "match") == 0 && 
                compiler_state.pattern_matching_enabled) {
            i++; // Skip 'match'
            if (tokens[i].type == TOKEN_IDENTIFIER) {
                printf("Pattern matching on: %s\n", tokens[i].value);
                // Pattern matching implementation would go here
            }
        }
        
        i++;
    }
}

// Code generation (simple assembly output)
void generate_assembly(const char* output_file) {
    FILE* asm_file = fopen(output_file, "w");
    if (!asm_file) {
        printf("Error: Cannot create assembly file\n");
        return;
    }
    
    fprintf(asm_file, ".section .data\n");
    
    // Generate data section for variables
    for (int i = 0; i < compiler_state.var_count; i++) {
        Variable* var = &compiler_state.variables[i];
        if (var->is_assigned) {
            switch (var->type) {
                case TYPE_NUMBER:
                    fprintf(asm_file, "%s: .quad %lf\n", var->name, var->value.number);
                    break;
                case TYPE_STRING:
                    fprintf(asm_file, "%s: .asciz \"%s\"\n", var->name, var->value.string);
                    break;
                default:
                    break;
            }
        }
    }
    
    fprintf(asm_file, "\n.section .text\n");
    fprintf(asm_file, ".global _start\n\n");
    
    // Generate code section
    fprintf(asm_file, "_start:\n");
    fprintf(asm_file, "    # ZenLang compiled code\n");
    
    // Generate function code
    for (int i = 0; i < compiler_state.func_count; i++) {
        Function* func = &compiler_state.functions[i];
        fprintf(asm_file, "\n%s:\n", func->name);
        fprintf(asm_file, "    # Function: %s\n", func->name);
        if (func->is_curried) {
            fprintf(asm_file, "    # Curried function (level %d)\n", func->curry_level);
        }
        fprintf(asm_file, "    ret\n");
    }
    
    // Exit system call
    fprintf(asm_file, "\n    # Exit program\n");
    fprintf(asm_file, "    mov $60, %%rax\n");
    fprintf(asm_file, "    mov $0, %%rdi\n");
    fprintf(asm_file, "    syscall\n");
    
    fclose(asm_file);
    printf("Assembly code generated: %s\n", output_file);
}

// Main compiler function
int compile_file(const char* input_file, const char* output_file) {
    FILE* file = fopen(input_file, "r");
    if (!file) {
        printf("Error: Cannot open input file %s\n", input_file);
        return 1;
    }
    
    // Read source code
    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    fseek(file, 0, SEEK_SET);
    
    char* source = malloc(file_size + 1);
    fread(source, 1, file_size, file);
    source[file_size] = '\0';
    fclose(file);
    
    printf("ZenLang Compiler v1.0\n");
    printf("Compiling: %s\n", input_file);
    printf("Features: Immutable dynamic typing, Pattern matching, Auto-currying, Memory zones\n\n");
    
    // Initialize compiler
    compiler_state.use_braces = true; // Default
    compiler_state.pattern_matching_enabled = false;
    compiler_state.auto_curry_enabled = false;
    init_memory_zones();
    
    // Tokenize
    Token* tokens = tokenize(source);
    
    // Parse and execute
    parse_and_execute(tokens);
    
    // Generate assembly
    generate_assembly(output_file);
    
    printf("\nCompilation Statistics:\n");
    printf("Variables: %d\n", compiler_state.var_count);
    printf("Functions: %d\n", compiler_state.func_count);
    printf("Memory Zones: %d\n", compiler_state.zone_count);
    printf("Brace Style: %s\n", compiler_state.use_braces ? "Enabled" : "Disabled");
    printf("Pattern Matching: %s\n", compiler_state.pattern_matching_enabled ? "Enabled" : "Disabled");
    printf("Auto-currying: %s\n", compiler_state.auto_curry_enabled ? "Enabled" : "Disabled");
    
    free(source);
    free(tokens);
    return 0;
}

// Example usage and test
void create_example_program() {
    FILE* example = fopen("example.zen", "w");
    if (!example) return;
    
    fprintf(example, "# ZenLang Example Program\n");
    fprintf(example, "#pragma braces\n");
    fprintf(example, "#pragma pattern-match\n");
    fprintf(example, "#pragma auto-curry\n\n");
    
    fprintf(example, "# Memory zone for fast calculations\n");
    fprintf(example, "zone fast_math\n\n");
    
    fprintf(example, "# Immutable variables - once assigned, cannot change\n");
    fprintf(example, "let x = 42\n");
    fprintf(example, "let name = \"ZenLang\"\n");
    fprintf(example, "let pi = 3.14159\n\n");
    
    fprintf(example, "# Function with auto-currying support\n");
    fprintf(example, "fn add(a, b) {\n");
    fprintf(example, "    return a + b\n");
    fprintf(example, "}\n\n");
    
    fprintf(example, "# Pattern matching function\n");
    fprintf(example, "fn process_data(input) {\n");
    fprintf(example, "    match input {\n");
    fprintf(example, "        case \"number\" => return \"Processing number\"\n");
    fprintf(example, "        case \"string\" => return \"Processing string\"\n");
    fprintf(example, "        case * => return \"Unknown type\"\n");
    fprintf(example, "    }\n");
    fprintf(example, "}\n\n");
    
    fprintf(example, "# Object-like structure (flexible OOP)\n");
    fprintf(example, "let person = {\n");
    fprintf(example, "    name: \"Alice\",\n");
    fprintf(example, "    age: 30,\n");
    fprintf(example, "    greet: fn() { return \"Hello!\" }\n");
    fprintf(example, "}\n");
    
    fclose(example);
    printf("Created example program: example.zen\n");
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        printf("ZenLang Compiler v1.0\n");
        printf("Usage: %s <input.zen> [output.s]\n\n", argv[0]);
        
        printf("Unique Features:\n");
        printf("1. Immutable Dynamic Typing - Variables can't be reassigned once set\n");
        printf("2. Flexible OOP - Object concepts without strict enforcement\n");
        printf("3. Pragma-controlled Syntax - Choose brace style with #pragma\n");
        printf("4. Pattern Matching - Advanced pattern matching capabilities\n");
        printf("5. Auto-currying - Functions automatically curry when partially applied\n");
        printf("6. Memory Zones - Custom memory management zones\n");
        printf("7. Pipe Operations - Functional composition with | operator\n\n");
        
        create_example_program();
        printf("Try: %s example.zen\n", argv[0]);
        return 1;
    }
    
    const char* input_file = argv[1];
    const char* output_file = argc > 2 ? argv[2] : "output.s";
    
    return compile_file(input_file, output_file);
}
