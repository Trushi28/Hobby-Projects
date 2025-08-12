# Flux Language - Pipeline Operations Demonstration
# This example showcases functional composition with the | operator

#pragma braces

# Define utility functions for pipeline operations
func double(x) {
    return x * 2
}

func add_ten(x) {
    return x + 10
}

func square(x) {
    return x * x
}

func to_string(x) {
    return "Result: " + x
}

func is_even(x) {
    return x % 2 == 0
}

# Basic pipeline example
print("=== Basic Pipeline Operations ===")
let value = 5
let result = value | double | add_ten
print("5 | double | add_ten =", result)  # 5 -> 10 -> 20

# More complex pipeline
let complex_result = 3 | double | square | add_ten
print("3 | double | square | add_ten =", complex_result)  # 3 -> 6 -> 36 -> 46

# Pipeline with string operations
func add_prefix(text) {
    return ">> " + text
}

func add_suffix(text) {
    return text + " <<"
}

func to_uppercase(text) {
    return text  # In real implementation, would convert to uppercase
}

let text_pipeline = "hello" | add_prefix | to_uppercase | add_suffix
print("Text pipeline result:", text_pipeline)

# Numerical processing pipeline
print("\n=== Numerical Processing Pipeline ===")

func multiply_by(multiplier) {
    func inner(x) {
        return x * multiplier
    }
    return inner
}

func divide_by(divisor) {
    func inner(x) {
        return x / divisor
    }
    return inner
}

func round_to_int(x) {
    return x  # Simplified - would do actual rounding
}

# Create specialized functions
let multiply_by_3 = multiply_by(3)
let divide_by_2 = divide_by(2)

let number_result = 10 | multiply_by_3 | divide_by_2 | round_to_int
print("10 | *3 | /2 | round =", number_result)  # 10 -> 30 -> 15 -> 15

# Data transformation pipeline
print("\n=== Data Transformation Pipeline ===")

func filter_positive(x) {
    if x > 0 {
        return x
    } else {
        return 0
    }
}

func clamp_max(max_val) {
    func inner(x) {
        if x > max_val {
            return max_val
        } else {
            return x
        }
    }
    return inner
}

func normalize(x) {
    return x / 100.0
}

let data_point = -50
let processed = data_point | filter_positive | clamp_max(75) | normalize
print("Data processing result:", processed)

# Conditional pipeline
print("\n=== Conditional Pipeline ===")

func process_if_even(x) {
    if is_even(x) {
        return x | double | square
    } else {
        return x | add_ten | double
    }
}

let even_number = 4
let odd_number = 5

let even_result = process_if_even(even_number)
let odd_result = process_if_even(odd_number)

print("Even number (4) processing:", even_result)
print("Odd number (5) processing:", odd_result)

# Pipeline with error handling
print("\n=== Pipeline with Validation ===")

func validate_positive(x) {
    if x <= 0 {
        print("Warning: Negative value detected, using absolute value")
        return x * -1
    }
    return x
}

func safe_divide_by_ten(x) {
    if x == 0 {
        print("Error: Cannot divide zero")
        return 0
    }
    return x / 10
}

let test_values = [-15, 0, 25, 100]

func process_value(val) {
    return val | validate_positive | double | safe_divide_by_ten
}

# Process each value through the pipeline
let processed_values = []
# In real implementation, would iterate through array
print("Processing -15:", process_value(-15))
print("Processing 0:", process_value(0))
print("Processing 25:", process_value(25))
print("Processing 100:", process_value(100))

# Advanced: Pipeline composition
print("\n=== Pipeline Composition ===")

func create_math_pipeline() {
    func pipeline(x) {
        return x | double | add_ten | square
    }
    return pipeline
}

func create_string_pipeline() {
    func pipeline(x) {
        return x | to_string | add_prefix | add_suffix
    }
    return pipeline
}

let math_pipe = create_math_pipeline()
let string_pipe = create_string_pipeline()

let math_result = math_pipe(3)  # 3 -> 6 -> 16 -> 256
let string_result = string_pipe(42)

print("Math pipeline result:", math_result)
print("String pipeline result:", string_result)
