# ZenLang - Auto-Currying Demonstration
# This example showcases the unique auto-currying feature

#pragma braces
#pragma auto-curry

# Basic auto-currying example
fn add(a, b) {
    return a + b
}

# When we provide fewer arguments than required, auto-currying kicks in
let add5 = add(5)        # Creates a curried function that adds 5
let result1 = add5(10)   # Applies the remaining argument: 5 + 10 = 15

print("add(5)(10) =", result1)

# Three-parameter function
fn multiply(a, b, c) {
    return a * b * c
}

# Progressive currying
let double = multiply(2)          # Curried: needs b and c
let quadruple = double(2)         # Further curried: needs c
let result2 = quadruple(5)        # Final application: 2 * 2 * 5 = 20

print("multiply(2)(2)(5) =", result2)

# Alternative currying paths
let triple = multiply(3)          # Curried: needs b and c
let result3 = triple(4, 2)        # Apply remaining arguments at once: 3 * 4 * 2 = 24

print("multiply(3)(4, 2) =", result3)

# Currying with different types
fn format_message(prefix, name, suffix) {
    return prefix + " " + name + " " + suffix
}

let hello_formatter = format_message("Hello")
let greet_john = hello_formatter("John")
let final_message = greet_john("!")

print("Formatted message:", final_message)

# Currying for specialized functions
fn power(base, exponent) {
    let result = 1
    let i = 0
    while i < exponent {
        result = result * base
        i = i + 1
    }
    return result
}

let square = power(*, 2)         # Curry second parameter, * means "to be provided"
let cube = power(*, 3)           # Curry second parameter

print("square(4) =", square(4))  # 4^2 = 16
print("cube(3) =", cube(3))      # 3^3 = 27

# Currying with conditional logic
fn calculate_discount(customer_type, purchase_amount, discount_rate) {
    if customer_type == "premium" {
        return purchase_amount * (1 - discount_rate - 0.1)  # Extra 10% off
    } else if customer_type == "regular" {
        return purchase_amount * (1 - discount_rate)
    } else {
        return purchase_amount  # No discount for new customers
    }
}

let premium_calculator = calculate_discount("premium")
let regular_calculator = calculate_discount("regular")

# Apply different discount rates
let premium_5_percent = premium_calculator(*, 0.05)
let regular_10_percent = regular_calculator(*, 0.10)

print("Premium customer, $100, 5% discount:", premium_5_percent(100))
print("Regular customer, $100, 10% discount:", regular_10_percent(100))

# Advanced currying: Function composition
fn compose(f, g, x) {
    return f(g(x))
}

fn double_value(x) {
    return x * 2
}

fn add_ten(x) {
    return x + 10
}

# Create composed function using currying
let double_then_add_ten = compose(add_ten, double_value)
print("compose(add_ten, double_value)(5) =", double_then_add_ten(5))  # (5*2)+10 = 20

# Partial application with arrays (conceptual)
fn sum_three(a, b, c) {
    return a + b + c
}

fn multiply_and_sum(multiplier, a, b, c) {
    return (a * multiplier) + (b * multiplier) + (c * multiplier)
}

# Create specialized functions
let sum_with_first_10 = sum_three(10)      # Needs b and c
let double_and_sum = multiply_and_sum(2)   # Needs a, b, c

print("sum_three(10)(5, 7) =", sum_with_first_10(5, 7))       # 10+5+7 = 22
print("multiply_and_sum(2)(3, 4, 5) =", double_and_sum(3, 4, 5)) # (3*2)+(4*2)+(5*2) = 24

# Currying with recursion
fn factorial(n, acc) {
    if n <= 1 {
        return acc
    } else {
        return factorial(n - 1, acc * n)
    }
}

# Curry the accumulator parameter for tail-recursive factorial
let factorial_from_1 = factorial(*, 1)

print("factorial(5) =", factorial_from_1(5))
print("factorial(6) =", factorial_from_1(6))

# Event handler currying
fn create_handler(event_type, element_id, action) {
    print("Handler created for", event_type, "on", element_id, ":", action)
    return "handler_" + event_type + "_" + element_id
}

# Create specialized handler creators
let click_handler_creator = create_handler("click")
let hover_handler_creator = create_handler("hover")

# Create specific handlers
let button_click = click_handler_creator("button1", "submit_form")
let menu_hover = hover_handler_creator("menu", "show_dropdown")

# Validation function with currying
fn validate(rule_type, min_value, max_value, input_value) {
    if rule_type == "range" {
        return input_value >= min_value && input_value <= max_value
    } else if rule_type == "min" {
        return input_value >= min_value
    } else if rule_type == "max" {
        return input_value <= max_value
    }
    return false
}

# Create specialized validators
let age_validator = validate("range", 18, 65)
let score_validator = validate("range", 0, 100)
let positive_validator = validate("min", 0, 999999)  # Large max value

print("Age validation (25):", age_validator(25))        # true
print("Age validation (15):", age_validator(15))        # false
print("Score validation (85):", score_validator(85))    # true
print("Positive validation (-5):", positive_validator(-5)) # false

# Higher-order currying
fn create_operation(op_type) {
    if op_type == "math" {
        fn math_operation(operation, a, b) {
            if operation == "add" {
                return a + b
            } else if operation == "multiply" {
                return a * b
            } else if operation == "subtract" {
                return a - b
            }
            return 0
        }
        return math_operation
    } else if op_type == "string" {
        fn string_operation(operation, a, b) {
            if operation == "concat" {
                return a + b
            } else if operation == "repeat" {
                let result = ""
                let i = 0
                while i < b {
                    result = result + a
                    i = i + 1
                }
                return result
            }
            return a
        }
        return string_operation
    }
}

# Create operation categories
let math_ops = create_operation("math")
let string_ops = create_operation("string")

# Create specific operations
let math_adder = math_ops("add")
let string_repeater = string_ops("repeat")

print("Math add(10, 5):", math_adder(10, 5))           # 15
print("String repeat('Hi', 3):", string_repeater("Hi", 3))  # HiHiHi

print("\n=== Auto-currying Summary ===")
print("✓ Functions automatically curry when given fewer arguments")
print("✓ Curried functions can be stored and reused")
print("✓ Multiple currying levels supported")
print("✓ Works with recursion and higher-order functions")
print("✓ Enables powerful function composition patterns")
