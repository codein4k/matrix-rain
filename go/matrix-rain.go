package main

import (
	"fmt"
	"math/rand"
	"os"
	"os/exec"
	"strconv"
	"strings"
	"time"
)

const REFRESH_RATE = 40 * time.Millisecond

type Drop struct {
	x         int
	y         float32
	speed     float32
	trail_len int
}

func getTerminalSize() (int, int) {
	cmd := exec.Command("sh", "-c", "stty size < /dev/tty")
	out, err := cmd.Output()
	if err != nil {
		return 80, 24
	}

	parts := strings.Fields(string(out))
	if len(parts) != 2 {
		return 80, 24
	}

	rows, _ := strconv.Atoi(parts[0])
	cols, _ := strconv.Atoi(parts[1])

	return cols, rows
}

func initDrops(cols int) []Drop {
	drops := make([]Drop, cols)

	for i := 0; i < cols; i++ {
		drops[i] = Drop{
			x:         i + 1,
			y:         1,
			speed:     0.1 + float32(rand.Intn(100))/100,
			trail_len: 10 + rand.Intn(10),
		}
	}

	return drops
}

func printPixel(y, x int, r, g, b int, c byte) {
	if y < 1 {
		return
	}

	fmt.Printf("\x1b[%d;%dH\x1b[38;2;%d;%d;%dm%c", y, x, r, g, b, c)
}

func getChar() byte {
	if rand.Intn(2) == 0 {
		return '0'
	}
	return '1'
}

func main() {
	rand.Seed(time.Now().UnixNano())

	width, height := getTerminalSize()
	drops := initDrops(width)

	// hide cursor + clear screen
	fmt.Print("\x1b[?25l\x1b[2J")

	for {
		newWidth, newHeight := getTerminalSize()

		if newWidth != width || newHeight != height {
			width = newWidth
			height = newHeight
			drops = initDrops(width)
			fmt.Print("\x1b[2J")
		}

		for i := 0; i < width; i++ {
			d := &drops[i]
			headY := int(d.y)

			if headY >= 1 && headY <= height {
				printPixel(headY, d.x, 150, 255, 150, getChar())
			}

			for j := 1; j <= d.trail_len; j++ {
				ty := headY - j
				if ty < 1 || ty > height {
					continue
				}

				ratio := 1.0 - float32(j)/float32(d.trail_len)
				green := int(255 * ratio)

				printPixel(ty, d.x, 0, green, 0, getChar())
			}

			d.y += d.speed

			if d.y-float32(d.trail_len) > float32(height) {
				d.y = 0
				d.speed = 0.1 + float32(rand.Intn(100))/100
			}
		}

		os.Stdout.Sync()

		time.Sleep(REFRESH_RATE)
	}
}
