#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <netinet/in.h>

#define PORT 8080
#define RESPONSE "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n" \
                 "<html><body><h1>Hello from C HTTP Server!</h1></body></html>"

int main() {
    int server_fd, client_fd;
    struct sockaddr_in server_addr;
    char buffer[3000];

    server_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (server_fd == 0) {
        perror("socket failed");
        exit(EXIT_FAILURE);
    }

    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    if (bind(server_fd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("bind failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    if (listen(server_fd, 3) < 0) {
        perror("listen failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    printf("Server is running on http://localhost:%d\n", PORT);

    while (1) {
        client_fd = accept(server_fd, NULL, NULL);
        if (client_fd < 0) {
            perror("accept failed");
            continue;
        }

        read(client_fd, buffer, sizeof(buffer) - 1);
        write(client_fd, RESPONSE, strlen(RESPONSE));
        close(client_fd);
    }

    return 0;
}

