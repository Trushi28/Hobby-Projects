#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <netinet/in.h>
#include <time.h>

#define PORT 8080

int generate_number() {
    srand(time(NULL));
    return (rand() % 10) + 1;
}

void send_response(int client_fd, int target, int guess) {
    char response[4096];
    char message[1024];

    if (guess == target) {
        sprintf(message, "<h1 style='color:green;'>🎉 Correct! You guessed %d!</h1>", guess);
    } else if (guess < target) {
        sprintf(message, "<h1 style='color:orange;'>⬆️ Too low! Try higher than %d</h1>", guess);
    } else if (guess > target) {
        sprintf(message, "<h1 style='color:red;'>⬇️ Too high! Try lower than %d</h1>", guess);
    } else {
        sprintf(message, "<h1 style='color:gray;'>🤔 Invalid guess!</h1>");
    }

    snprintf(response, sizeof(response),
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n"
        "<html><head><title>Guess Game</title></head>"
        "<body style='background-color: #111; color: white; font-family: Arial; text-align: center; padding-top: 50px;'>"
        "<h2 style='color:cyan;'>🎮 Guess the Number (1 to 10)</h2>"
        "%s"
        "<form method='get'>"
        "<input type='number' name='guess' min='1' max='10' required>"
        "<button type='submit' style='padding:10px 20px; background-color:purple; color:white; border:none;'>Guess</button>"
        "</form>"
        "<p style='color:gray;'>Refresh to start over with a new number</p>"
        "</body></html>",
        message
    );

    write(client_fd, response, strlen(response));
}

int parse_guess(char *buffer) {
    char *start = strstr(buffer, "GET /?guess=");
    if (!start) return -1;

    int guess = atoi(start + 12);
    return guess;
}

int main() {
    int server_fd, client_fd;
    struct sockaddr_in address;
    char buffer[4096];
    int number_to_guess = generate_number();

    server_fd = socket(AF_INET, SOCK_STREAM, 0);
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(PORT);

    bind(server_fd, (struct sockaddr *)&address, sizeof(address));
    listen(server_fd, 5);

    printf("🎯 HTTP Guess Game started on http://localhost:%d\n", PORT);

    while (1) {
        client_fd = accept(server_fd, NULL, NULL);
        if (client_fd < 0) continue;

        memset(buffer, 0, sizeof(buffer));
        read(client_fd, buffer, sizeof(buffer) - 1);

        int guess = parse_guess(buffer);
        send_response(client_fd, number_to_guess, guess);

        close(client_fd);
    }

    return 0;
}

