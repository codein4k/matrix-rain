import os
import sys
import time
import random
import shutil

REFRESH_RATE = 0.04

class Drop:
    def __init__(self):
        self.x = 0
        self.y = 0.0
        self.speed = 0.0
        self.trail_len = 0


def get_terminal_size():
    size = shutil.get_terminal_size()
    return size.columns, size.lines


def init_drops(drops, cols):
    for i in range(cols):
        d = drops[i]
        d.x = i + 1
        d.y = 1
        d.speed = 0.1 + random.randint(0, 99) / 100
        d.trail_len = 10 + random.randint(0, 9)


def print_pixel(y, x, r, g, b, c):
    if y < 1:
        return
    sys.stdout.write(f"\033[{y};{x}H\033[38;2;{r};{g};{b}m{c}")


def get_char():
    return random.choice("01")


def main():
    width, height = get_terminal_size()

    random.seed()

    drops = [Drop() for _ in range(width)]
    init_drops(drops, width)

    # hide cursor + clear screen
    sys.stdout.write("\033[?25l\033[2J")
    sys.stdout.flush()

    while True:
        new_width, new_height = get_terminal_size()

        if new_width != width or new_height != height:
            width, height = new_width, new_height
            drops = [Drop() for _ in range(width)]
            init_drops(drops, width)
            sys.stdout.write("\033[2J")

        for i in range(width):
            drop = drops[i]
            head_y = int(drop.y)

            if 1 <= head_y <= height:
                print_pixel(head_y, drop.x, 150, 255, 150, get_char())

            for j in range(1, drop.trail_len + 1):
                ty = head_y - j
                if ty < 1 or ty > height:
                    continue

                ratio = 1.0 - (j / drop.trail_len)
                green = int(255 * ratio)

                print_pixel(ty, drop.x, 0, green, 0, get_char())

            # drop movement
            drop.y += drop.speed

            # reset
            if drop.y - drop.trail_len > height:
                drop.y = 0
                drop.speed = 0.1 + random.randint(0, 99) / 100

        sys.stdout.flush()
        time.sleep(REFRESH_RATE)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        # show cursor again on exit
        sys.stdout.write("\033[?25h")
        sys.stdout.flush()