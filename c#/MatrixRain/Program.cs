using System;
using System.Threading;

class Drop
{
    public int x;
    public float y;
    public float speed;
    public int trail_len;
}

class Program
{
    const int REFRESH_RATE = 40; // ms
    static Random rand = new Random();

    static int width, height;
    static Drop[] drops;

    static void GetTerminalSize(out int col, out int row)
    {
        col = Console.WindowWidth;
        row = Console.WindowHeight;
    }

    static void InitDrops(int cols)
    {
        drops = new Drop[cols];

        for (int i = 0; i < cols; i++)
        {
            drops[i] = new Drop
            {
                x = i + 1,
                y = 1,
                speed = 0.1f + rand.Next(100) / 100f,
                trail_len = 10 + rand.Next(10)
            };
        }
    }

    static void PrintPixel(int y, int x, int r, int g, int b, char c)
    {
        if (y < 1) return;

        // ANSI: move cursor + set RGB color + print char
        Console.Write($"\x1b[{y};{x}H\x1b[38;2;{r};{g};{b}m{c}");
    }

    static char GetChar()
    {
        return rand.Next(2) == 0 ? '0' : '1';
    }

    static void Main()
    {
        // پیشنهاد: خروجی بافر نشه تا فریم‌ها سریع‌تر دیده بشن
        Console.OutputEncoding = System.Text.Encoding.UTF8;

        GetTerminalSize(out width, out height);
        InitDrops(width);

        // hide cursor + clear screen
        Console.Write("\x1b[?25l\x1b[2J");

        while (true)
        {
            int newWidth, newHeight;
            GetTerminalSize(out newWidth, out newHeight);

            if (newWidth != width || newHeight != height)
            {
                width = newWidth;
                height = newHeight;

                InitDrops(width);
                Console.Write("\x1b[2J");
            }

            for (int i = 0; i < width; i++)
            {
                var d = drops[i];
                int head_y = (int)d.y;

                if (head_y >= 1 && head_y <= height)
                {
                    PrintPixel(head_y, d.x, 150, 255, 150, GetChar());
                }

                for (int j = 1; j <= d.trail_len; j++)
                {
                    int ty = head_y - j;
                    if (ty < 1 || ty > height) continue;

                    float ratio = 1.0f - ((float)j / d.trail_len);
                    int green = (int)(255 * ratio);

                    PrintPixel(ty, d.x, 0, green, 0, GetChar());
                }

                d.y += d.speed;

                if (d.y - d.trail_len > height)
                {
                    d.y = 0;
                    d.speed = 0.1f + rand.Next(100) / 100f;
                }
            }

            Thread.Sleep(REFRESH_RATE);
        }
    }
}