#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <time.h>
#include <sys/ioctl.h>

#define REFRESH_RATE    40000

typedef struct{
    int x;
    float y;
    float speed;
    int trail_len;
} Drop;

void get_terminal_size(int *col, int *row)
{
    struct winsize w;
    ioctl(STDOUT_FILENO, TIOCGWINSZ, &w);
    *col = w.ws_col;
    *row = w.ws_row;
}

void init_drops(Drop *drops, int cols)
{
    for(int i=0; i < cols; i++)
    {
        drops[i].x = i + 1;
        drops[i].y = 1;
        drops[i].speed = .1 + (float)(rand() % 100) / 100;
        drops[i].trail_len = 10 + rand() % 10;
    }
}

void print_pixel(int y, int x, int r, int g, int b, char c)
{
    if(y < 1) return;

    printf("\033[%d;%dH\033[38;2;%d;%d;%dm%c", y, x, r, g, b, c);
}

char get_char() {
    //return (char)(33 + rand() % 94);    // ASCII 33-126
    return "01"[rand() % 2];
}

int main()
{
    int width, height;
    get_terminal_size(&width, &height);

    srand(time(NULL));

    Drop *drops = malloc(width * sizeof(Drop));
    //init drops
    init_drops(drops, width);

    printf("\033[?25l\033[2J");

    int new_width, new_height;
    while (1)
    {
        //check terminal size for any changes by user
        get_terminal_size(&new_width, &new_height);
        if(new_width != width || new_height != height)
        {
            width = new_width; height = new_height;
            drops = realloc(drops, width * sizeof(Drop));
            init_drops(drops, width);
            printf("\033[2J");
        }

        for(int i=0; i < width; i++)
        {
            int head_y = (int)drops[i].y;

            if(head_y >=1 && head_y <= height){
                //print pixel
                print_pixel(head_y, drops[i].x, 150, 255, 150, get_char());
            }

            for(int j=1; j <= drops[i].trail_len; j++)
            {
                int ty = head_y - j;
                if(ty < 1 || ty > height) continue;

                float ratio = 1.0f - ((float)j / drops[i].trail_len);
                int green = 0;

                green = (int)(255 * ratio);

                print_pixel(ty, drops[i].x, 0, green, 0, get_char());
            }

            //move drop down
            drops[i].y += drops[i].speed;

            //reset the drop if it is completely off the screen
            if(drops[i].y - drops[i].trail_len > height){
                drops[i].y = 0;
                drops[i].speed = .1 + (float)(rand() % 100) / 100;
            }
        }

        usleep(REFRESH_RATE);
    }
    

    return 0;
}