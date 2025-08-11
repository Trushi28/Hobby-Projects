#include <gtk/gtk.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>

#define GRID_SIZE 4
#define BUTTON_SIZE 80

typedef struct {
    GtkWidget *window;
    GtkWidget *grid;
    GtkWidget *buttons[GRID_SIZE][GRID_SIZE];
    GtkWidget *status_label;
    GtkWidget *moves_label;
    GtkWidget *time_label;
    int puzzle[GRID_SIZE][GRID_SIZE];
    int empty_row, empty_col;
    int moves;
    time_t start_time;
    guint timer_id;
} GameData;

// Function prototypes
void init_puzzle(GameData *game);
void shuffle_puzzle(GameData *game);
void update_display(GameData *game);
gboolean check_win(GameData *game);
void on_button_clicked(GtkWidget *widget, gpointer data);
void on_new_game_clicked(GtkWidget *widget, gpointer data);
void on_solve_clicked(GtkWidget *widget, gpointer data);
gboolean update_timer(gpointer data);
void show_win_dialog(GameData *game);

// Initialize the puzzle in solved state
void init_puzzle(GameData *game) {
    int num = 1;
    for (int i = 0; i < GRID_SIZE; i++) {
        for (int j = 0; j < GRID_SIZE; j++) {
            if (i == GRID_SIZE - 1 && j == GRID_SIZE - 1) {
                game->puzzle[i][j] = 0; // Empty space
                game->empty_row = i;
                game->empty_col = j;
            } else {
                game->puzzle[i][j] = num++;
            }
        }
    }
}

// Shuffle the puzzle by making random valid moves
void shuffle_puzzle(GameData *game) {
    srand(time(NULL));
    
    // Make 1000 random valid moves to shuffle
    for (int i = 0; i < 1000; i++) {
        int direction = rand() % 4; // 0=up, 1=down, 2=left, 3=right
        int new_row = game->empty_row;
        int new_col = game->empty_col;
        
        switch (direction) {
            case 0: new_row--; break; // Move empty up (tile down)
            case 1: new_row++; break; // Move empty down (tile up)
            case 2: new_col--; break; // Move empty left (tile right)
            case 3: new_col++; break; // Move empty right (tile left)
        }
        
        // Check if move is valid
        if (new_row >= 0 && new_row < GRID_SIZE && 
            new_col >= 0 && new_col < GRID_SIZE) {
            // Swap empty space with the tile
            game->puzzle[game->empty_row][game->empty_col] = 
                game->puzzle[new_row][new_col];
            game->puzzle[new_row][new_col] = 0;
            game->empty_row = new_row;
            game->empty_col = new_col;
        }
    }
}

// Update the visual display
void update_display(GameData *game) {
    for (int i = 0; i < GRID_SIZE; i++) {
        for (int j = 0; j < GRID_SIZE; j++) {
            if (game->puzzle[i][j] == 0) {
                gtk_button_set_label(GTK_BUTTON(game->buttons[i][j]), "");
                gtk_widget_set_sensitive(game->buttons[i][j], FALSE);
                
                // Style empty button
                GtkStyleContext *context = gtk_widget_get_style_context(game->buttons[i][j]);
                gtk_style_context_add_class(context, "empty-tile");
            } else {
                char label[10];
                sprintf(label, "%d", game->puzzle[i][j]);
                gtk_button_set_label(GTK_BUTTON(game->buttons[i][j]), label);
                gtk_widget_set_sensitive(game->buttons[i][j], TRUE);
                
                // Remove empty style and add number style
                GtkStyleContext *context = gtk_widget_get_style_context(game->buttons[i][j]);
                gtk_style_context_remove_class(context, "empty-tile");
                gtk_style_context_add_class(context, "number-tile");
            }
        }
    }
    
    // Update moves counter
    char moves_text[50];
    sprintf(moves_text, "Moves: %d", game->moves);
    gtk_label_set_text(GTK_LABEL(game->moves_label), moves_text);
}

// Check if puzzle is solved
gboolean check_win(GameData *game) {
    int expected = 1;
    for (int i = 0; i < GRID_SIZE; i++) {
        for (int j = 0; j < GRID_SIZE; j++) {
            if (i == GRID_SIZE - 1 && j == GRID_SIZE - 1) {
                if (game->puzzle[i][j] != 0) return FALSE;
            } else {
                if (game->puzzle[i][j] != expected++) return FALSE;
            }
        }
    }
    return TRUE;
}

// Handle button clicks
void on_button_clicked(GtkWidget *widget, gpointer data) {
    GameData *game = (GameData *)data;
    
    // Find which button was clicked
    int clicked_row = -1, clicked_col = -1;
    for (int i = 0; i < GRID_SIZE; i++) {
        for (int j = 0; j < GRID_SIZE; j++) {
            if (game->buttons[i][j] == widget) {
                clicked_row = i;
                clicked_col = j;
                break;
            }
        }
        if (clicked_row != -1) break;
    }
    
    // Check if the clicked tile is adjacent to empty space
    int row_diff = abs(clicked_row - game->empty_row);
    int col_diff = abs(clicked_col - game->empty_col);
    
    if ((row_diff == 1 && col_diff == 0) || (row_diff == 0 && col_diff == 1)) {
        // Valid move - swap the tile with empty space
        game->puzzle[game->empty_row][game->empty_col] = 
            game->puzzle[clicked_row][clicked_col];
        game->puzzle[clicked_row][clicked_col] = 0;
        game->empty_row = clicked_row;
        game->empty_col = clicked_col;
        
        game->moves++;
        update_display(game);
        
        // Check for win
        if (check_win(game)) {
            g_source_remove(game->timer_id);
            show_win_dialog(game);
        }
    }
}

// Timer update function
gboolean update_timer(gpointer data) {
    GameData *game = (GameData *)data;
    time_t current_time = time(NULL);
    int elapsed = (int)difftime(current_time, game->start_time);
    
    char time_text[50];
    int minutes = elapsed / 60;
    int seconds = elapsed % 60;
    sprintf(time_text, "Time: %02d:%02d", minutes, seconds);
    gtk_label_set_text(GTK_LABEL(game->time_label), time_text);
    
    return TRUE; // Continue timer
}

// Show win dialog
void show_win_dialog(GameData *game) {
    time_t end_time = time(NULL);
    int elapsed = (int)difftime(end_time, game->start_time);
    int minutes = elapsed / 60;
    int seconds = elapsed % 60;
    
    char message[200];
    sprintf(message, "🎉 Congratulations! 🎉\n\nYou solved the puzzle!\n\nMoves: %d\nTime: %02d:%02d", 
            game->moves, minutes, seconds);
    
    GtkWidget *dialog = gtk_message_dialog_new(GTK_WINDOW(game->window),
                                             GTK_DIALOG_DESTROY_WITH_PARENT,
                                             GTK_MESSAGE_INFO,
                                             GTK_BUTTONS_OK,
                                             "%s", message);
    
    gtk_window_set_title(GTK_WINDOW(dialog), "Puzzle Solved!");
    gtk_dialog_run(GTK_DIALOG(dialog));
    gtk_widget_destroy(dialog);
    
    gtk_label_set_text(GTK_LABEL(game->status_label), "🏆 Puzzle Solved! Click 'New Game' to play again.");
}

// New game button handler
void on_new_game_clicked(GtkWidget *widget, gpointer data) {
    GameData *game = (GameData *)data;
    
    // Stop current timer if running
    if (game->timer_id > 0) {
        g_source_remove(game->timer_id);
    }
    
    game->moves = 0;
    game->start_time = time(NULL);
    
    init_puzzle(game);
    shuffle_puzzle(game);
    update_display(game);
    
    gtk_label_set_text(GTK_LABEL(game->status_label), "🎯 Arrange numbers 1-15 in order. Click tiles adjacent to empty space!");
    gtk_label_set_text(GTK_LABEL(game->time_label), "Time: 00:00");
    
    // Start timer
    game->timer_id = g_timeout_add(1000, update_timer, game);
}

// Solve button handler (shows solution)
void on_solve_clicked(GtkWidget *widget, gpointer data) {
    GameData *game = (GameData *)data;
    
    // Stop timer
    if (game->timer_id > 0) {
        g_source_remove(game->timer_id);
    }
    
    // Set to solved state
    init_puzzle(game);
    update_display(game);
    
    gtk_label_set_text(GTK_LABEL(game->status_label), "✅ Puzzle solved! Click 'New Game' for a new challenge.");
}

// Apply custom CSS styling
void apply_styles() {
    GtkCssProvider *provider = gtk_css_provider_new();
    const char *css = 
        "window { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }"
        ".game-title { font-size: 28px; font-weight: bold; color: white; margin: 10px; }"
        ".status-label { font-size: 16px; color: white; margin: 5px; }"
        ".info-label { font-size: 14px; color: #f0f0f0; margin: 3px; }"
        ".number-tile { "
        "  font-size: 20px; font-weight: bold; "
        "  background: linear-gradient(145deg, #ffffff, #e6e6e6); "
        "  border: 2px solid #cccccc; border-radius: 8px; "
        "  color: #333333; min-width: 70px; min-height: 70px; "
        "}"
        ".number-tile:hover { "
        "  background: linear-gradient(145deg, #f0f8ff, #ddeeff); "
        "  border-color: #4a90e2; "
        "}"
        ".empty-tile { "
        "  background: rgba(255,255,255,0.1); "
        "  border: 2px dashed rgba(255,255,255,0.3); "
        "  border-radius: 8px; "
        "}"
        ".control-button { "
        "  font-size: 14px; font-weight: bold; "
        "  background: linear-gradient(145deg, #4a90e2, #357abd); "
        "  border: none; border-radius: 6px; color: white; "
        "  padding: 8px 16px; margin: 5px; "
        "}"
        ".control-button:hover { "
        "  background: linear-gradient(145deg, #357abd, #2968a3); "
        "}";
    
    gtk_css_provider_load_from_data(provider, css, -1, NULL);
    gtk_style_context_add_provider_for_screen(gdk_screen_get_default(),
                                            GTK_STYLE_PROVIDER(provider),
                                            GTK_STYLE_PROVIDER_PRIORITY_USER);
}

int main(int argc, char *argv[]) {
    gtk_init(&argc, &argv);
    
    GameData game = {0};
    
    // Apply custom styles
    apply_styles();
    
    // Create main window
    game.window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(game.window), "Sliding Block Puzzle");
    gtk_window_set_default_size(GTK_WINDOW(game.window), 400, 600);
    gtk_window_set_position(GTK_WINDOW(game.window), GTK_WIN_POS_CENTER);
    gtk_window_set_resizable(GTK_WINDOW(game.window), FALSE);
    
    // Create main container
    GtkWidget *main_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 10);
    gtk_container_add(GTK_CONTAINER(game.window), main_box);
    gtk_container_set_border_width(GTK_CONTAINER(main_box), 20);
    
    // Title
    GtkWidget *title = gtk_label_new("🧩 Sliding Block Puzzle");
    GtkStyleContext *title_context = gtk_widget_get_style_context(title);
    gtk_style_context_add_class(title_context, "game-title");
    gtk_box_pack_start(GTK_BOX(main_box), title, FALSE, FALSE, 0);
    
    // Status label
    game.status_label = gtk_label_new("🎯 Arrange numbers 1-15 in order. Click tiles adjacent to empty space!");
    gtk_label_set_line_wrap(GTK_LABEL(game.status_label), TRUE);
    gtk_label_set_justify(GTK_LABEL(game.status_label), GTK_JUSTIFY_CENTER);
    GtkStyleContext *status_context = gtk_widget_get_style_context(game.status_label);
    gtk_style_context_add_class(status_context, "status-label");
    gtk_box_pack_start(GTK_BOX(main_box), game.status_label, FALSE, FALSE, 0);
    
    // Info box for moves and time
    GtkWidget *info_box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 20);
    gtk_box_set_homogeneous(GTK_BOX(info_box), TRUE);
    
    game.moves_label = gtk_label_new("Moves: 0");
    game.time_label = gtk_label_new("Time: 00:00");
    
    GtkStyleContext *moves_context = gtk_widget_get_style_context(game.moves_label);
    GtkStyleContext *time_context = gtk_widget_get_style_context(game.time_label);
    gtk_style_context_add_class(moves_context, "info-label");
    gtk_style_context_add_class(time_context, "info-label");
    
    gtk_box_pack_start(GTK_BOX(info_box), game.moves_label, TRUE, TRUE, 0);
    gtk_box_pack_start(GTK_BOX(info_box), game.time_label, TRUE, TRUE, 0);
    gtk_box_pack_start(GTK_BOX(main_box), info_box, FALSE, FALSE, 0);
    
    // Create game grid
    game.grid = gtk_grid_new();
    gtk_grid_set_row_spacing(GTK_GRID(game.grid), 3);
    gtk_grid_set_column_spacing(GTK_GRID(game.grid), 3);
    gtk_widget_set_halign(game.grid, GTK_ALIGN_CENTER);
    
    // Create buttons
    for (int i = 0; i < GRID_SIZE; i++) {
        for (int j = 0; j < GRID_SIZE; j++) {
            game.buttons[i][j] = gtk_button_new();
            gtk_widget_set_size_request(game.buttons[i][j], BUTTON_SIZE, BUTTON_SIZE);
            gtk_grid_attach(GTK_GRID(game.grid), game.buttons[i][j], j, i, 1, 1);
            g_signal_connect(game.buttons[i][j], "clicked", 
                           G_CALLBACK(on_button_clicked), &game);
        }
    }
    
    gtk_box_pack_start(GTK_BOX(main_box), game.grid, TRUE, TRUE, 0);
    
    // Control buttons
    GtkWidget *button_box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 10);
    gtk_box_set_homogeneous(GTK_BOX(button_box), TRUE);
    
    GtkWidget *new_game_btn = gtk_button_new_with_label("🎮 New Game");
    GtkWidget *solve_btn = gtk_button_new_with_label("💡 Show Solution");
    
    GtkStyleContext *new_context = gtk_widget_get_style_context(new_game_btn);
    GtkStyleContext *solve_context = gtk_widget_get_style_context(solve_btn);
    gtk_style_context_add_class(new_context, "control-button");
    gtk_style_context_add_class(solve_context, "control-button");
    
    gtk_box_pack_start(GTK_BOX(button_box), new_game_btn, TRUE, TRUE, 0);
    gtk_box_pack_start(GTK_BOX(button_box), solve_btn, TRUE, TRUE, 0);
    gtk_box_pack_start(GTK_BOX(main_box), button_box, FALSE, FALSE, 0);
    
    // Connect signals
    g_signal_connect(new_game_btn, "clicked", G_CALLBACK(on_new_game_clicked), &game);
    g_signal_connect(solve_btn, "clicked", G_CALLBACK(on_solve_clicked), &game);
    g_signal_connect(game.window, "destroy", G_CALLBACK(gtk_main_quit), NULL);
    
    // Initialize game
    init_puzzle(&game);
    shuffle_puzzle(&game);
    update_display(&game);
    
    // Start timer
    game.start_time = time(NULL);
    game.timer_id = g_timeout_add(1000, update_timer, &game);
    
    // Show everything
    gtk_widget_show_all(game.window);
    
    // Start GTK main loop
    gtk_main();
    
    return 0;
}
