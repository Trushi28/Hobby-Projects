# Flux Language - Temporal Variables Demonstration
# This example showcases the unique temporal variable feature

#pragma braces

# Create a temporal variable to track temperature changes
temporal let temperature = 20.5
print("Initial temperature:", temperature)

# Update the temperature - this creates timeline entries
temperature = 25.0
print("Temperature updated to:", temperature)

temperature = 18.3
print("Temperature updated again to:", temperature)

temperature = 22.7
print("Final temperature:", temperature)

# Access historical values using temporal indexing
let temp_at_start = temperature[0]     # Gets value at timestamp 0
let temp_second = temperature[1]       # Gets value at timestamp 1
let temp_third = temperature[2]        # Gets value at timestamp 2
let current_temp = temperature         # Gets current value

print("Temperature history:")
print("  At start (t=0):", temp_at_start)
print("  Second update (t=1):", temp_second)
print("  Third update (t=2):", temp_third)
print("  Current value:", current_temp)

# Demonstrate temporal variable with different types
temporal let status = "initializing"
status = "running"
status = "completed"

print("Status history:")
print("  Initial:", status[0])
print("  Running:", status[1])
print("  Final:", status[2])

# Temporal variable with calculations
temporal let counter = 0
counter = counter + 1
counter = counter * 2
counter = counter + 5

print("Counter evolution:")
print("  Start:", counter[0])          # 0
print("  After +1:", counter[1])       # 1
print("  After *2:", counter[2])       # 2
print("  After +5:", counter[3])       # 7

# Example: Using temporal variables for debugging
func calculate_factorial(n) {
    temporal let result = 1
    temporal let step = 0
    
    while step < n {
        step = step + 1
        result = result * step
        print("Step", step, "- factorial so far:", result)
    }
    
    # Return both current result and history
    return result
}

let factorial_5 = calculate_factorial(5)
print("Final factorial result:", factorial_5)
