#include <gtk/gtk.h>

static void on_button_clicked(GtkWidget *widget, gpointer data) {
    gtk_label_set_text(GTK_LABEL(data), "You clicked the button! 🎉");
}

int main(int argc, char *argv[]) {
    gtk_init(&argc, &argv);

    GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "Frontend in C - GTK");
    gtk_window_set_default_size(GTK_WINDOW(window), 400, 200);
    gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);

    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 10);
    gtk_container_add(GTK_CONTAINER(window), box);

    GtkWidget *label = gtk_label_new("Hello! Click the button below 👇");
    gtk_box_pack_start(GTK_BOX(box), label, TRUE, TRUE, 10);

    GtkWidget *button = gtk_button_new_with_label("Click Me");
    g_signal_connect(button, "clicked", G_CALLBACK(on_button_clicked), label);
    gtk_box_pack_start(GTK_BOX(box), button, TRUE, TRUE, 10);

    g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), NULL);

    gtk_widget_show_all(window);
    gtk_main();

    return 0;
}

