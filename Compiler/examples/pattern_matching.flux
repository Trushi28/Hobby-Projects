# Flux Language - Advanced Pattern Matching Demonstration
# This example showcases the powerful pattern matching capabilities

#pragma braces

print("=== Basic Pattern Matching ===")

# Simple value matching
let status_code = 404
let response_message = match status_code {
    200 => "OK"
    201 => "Created"
    400 => "Bad Request"
    401 => "Unauthorized"
    403 => "Forbidden" 
    404 => "Not Found"
    500 => "Internal Server Error"
    default => "Unknown Status"
}

print("Status", status_code, ":", response_message)

# String pattern matching
let user_role = "admin"
let permissions = match user_role {
    "admin" => "Full access to all systems"
    "moderator" => "Can moderate content and users"
    "user" => "Basic user permissions"
    "guest" => "Read-only access"
    default => "No permissions"
}

print("Role:", user_role, "- Permissions:", permissions)

print("\n=== Type-based Pattern Matching ===")

func process_data(input) {
    let data_type = typeof(input)  # Hypothetical type checking
    return match data_type {
        "number" => {
            if input > 0 {
                return "Positive number: " + input
            } else if input < 0 {
                return "Negative number: " + input
            } else {
                return "Zero"
            }
        }
        "string" => {
            return "Text data: " + input
        }
        "boolean" => {
            return "Boolean value: " + input
        }
        default => {
            return "Unknown data type"
        }
    }
}

print(process_data(42))
print(process_data("Hello World"))
print(process_data(true))

print("\n=== Range Pattern Matching ===")

func categorize_score(score) {
    return match score {
        90 => "Perfect Score!"
        case x if x >= 80 => "Excellent (A)"
        case x if x >= 70 => "Good (B)"
        case x if x >= 60 => "Average (C)"
        case x if x >= 50 => "Below Average (D)"
        default => "Failing (F)"
    }
}

let test_scores = [95, 85, 75, 65, 55, 45]
# In real implementation, would iterate through array
print("Score 95:", categorize_score(95))
print("Score 85:", categorize_score(85))
print("Score 75:", categorize_score(75))
print("Score 65:", categorize_score(65))
print("Score 55:", categorize_score(55))
print("Score 45:", categorize_score(45))

print("\n=== Complex Pattern Matching with Objects ===")

func process_request(request) {
    return match request.method {
        "GET" => match request.path {
            "/api/users" => "Fetch all users"
            "/api/posts" => "Fetch all posts"
            case path if path.startswith("/api/user/") => "Fetch specific user"
            default => "GET: Unknown endpoint"
        }
        "POST" => match request.path {
            "/api/users" => "Create new user"
            "/api/posts" => "Create new post"
            "/api/login" => "User login"
            default => "POST: Unknown endpoint"
        }
        "PUT" => "Update resource"
        "DELETE" => "Delete resource"
        default => "Unsupported HTTP method"
    }
}

# Simulate request objects
let get_request = { method: "GET", path: "/api/users" }
let post_request = { method: "POST", path: "/api/login" }
let unknown_request = { method: "PATCH", path: "/api/test" }

print("GET /api/users:", process_request(get_request))
print("POST /api/login:", process_request(post_request))
print("PATCH /api/test:", process_request(unknown_request))

print("\n=== State Machine with Pattern Matching ===")

temporal let machine_state = "idle"

func process_event(event) {
    let current = machine_state
    
    let new_state = match current {
        "idle" => match event {
            "start" => "running"
            "configure" => "configuring"
            default => "idle"
        }
        "configuring" => match event {
            "save" => "idle"
            "cancel" => "idle"
            default => "configuring"
        }
        "running" => match event {
            "pause" => "paused"
            "stop" => "idle"
            "error" => "error"
            default => "running"
        }
        "paused" => match event {
            "resume" => "running"
            "stop" => "idle"
            default => "paused"
        }
        "error" => match event {
            "reset" => "idle"
            "restart" => "running"
            default => "error"
        }
        default => "idle"
    }
    
    machine_state = new_state
    return "State transition: " + current + " --(" + event + ")--> " + new_state
}

print("Initial state:", machine_state)
print(process_event("start"))
print(process_event("pause"))
print(process_event("resume"))
print(process_event("error"))
print(process_event("reset"))

print("\n=== Pattern Matching with Guards ===")

func analyze_number(x) {
    return match x {
        case n if n == 0 => "Zero - the neutral element"
        case n if n == 1 => "One - the multiplicative identity"
        case n if n > 0 && n % 2 == 0 => "Positive even number"
        case n if n > 0 && n % 2 != 0 => "Positive odd number"
        case n if n < 0 && n % 2 == 0 => "Negative even number"
        case n if n < 0 && n % 2 != 0 => "Negative odd number"
        case n if n > 100 => "Large number (>100)"
        case n if n < -100 => "Large negative number (<-100)"
        default => "Regular number"
    }
}

let numbers = [0, 1, 2, 3, -2, -3, 150, -150, 42]
# In real implementation, would iterate through array
print("Analysis of 0:", analyze_number(0))
print("Analysis of 1:", analyze_number(1))
print("Analysis of 2:", analyze_number(2))
print("Analysis of -3:", analyze_number(-3))
print("Analysis of 150:", analyze_number(150))

print("\n=== Nested Pattern Matching ===")

func process_api_response(response) {
    return match response.status {
        200 => match response.data.type {
            "user" => "User data: " + response.data.name
            "post" => "Post data: " + response.data.title
            "comment" => "Comment data: " + response.data.content
            default => "Unknown data type in successful response"
        }
        400 => "Bad request: " + response.error.message
        401 => "Authentication required"
        404 => match response.resource {
            "user" => "User not found"
            "post" => "Post not found"
            default => "Resource not found"
        }
        500 => "Server error: Please try again later"
        default => "Unexpected response status: " + response.status
    }
}

# Simulate API responses
let success_response = {
    status: 200,
    data: { type: "user", name: "John Doe" }
}

let not_found_response = {
    status: 404,
    resource: "user"
}

let server_error = {
    status: 500,
    error: { message: "Database connection failed" }
}

print("Success response:", process_api_response(success_response))
print("Not found response:", process_api_response(not_found_response))
print("Server error:", process_api_response(server_error))
