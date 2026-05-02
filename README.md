# 🟢 Matrix Rain — Terminal Animation

A beautiful **Matrix rain effect in the terminal**, built from scratch with a focus on **algorithm, animation logic, and terminal control**.

This project is not just about code — it's about understanding how to turn simple logic into a **real-time animated visual effect** using **ANSI escape codes**, **randomization**, and **low-level terminal manipulation**.

---

## 🎥 Tutorial Video

Video on youtube: [MatrixRain!](https://youtu.be/ueLN50HHhjw?si=xLoCj42JxLnuZfnf)

This repository is based on a full step-by-step tutorial where everything is explained in detail:

* How the **Matrix rain algorithm** works
* How to control the terminal using **ANSI escape codes**
* How to build smooth **terminal animations**
* How to generate **random ASCII characters**

---

## 🚀 Features

* 🌧️ Real-time **Matrix rain animation**
* 🎨 **TrueColor (24-bit RGB)** terminal output
* ⚡ Dynamic speed and tail length for each drop
* 📐 Automatically adapts to **terminal size changes**
* 🔁 Infinite animation loop with smooth rendering
* 🧠 Algorithm-first approach (language independent)

---

## 🧩 Implementations

This repository contains implementations of the Matrix rain effect in multiple programming languages

> The logic is the same across all versions — only syntax changes.

---

## 🛠️ How It Works

### 1. Terminal Size Detection

We detect terminal dimensions (rows & columns) using system-level tools like `ioctl`.

### 2. Raindrop Structure

Each raindrop has:

* Position (row, column)
* Speed (randomized)
* Tail length (randomized)

### 3. Memory Allocation

We allocate memory dynamically (e.g., using `malloc` in C) based on terminal width.

### 4. Randomization

* Speed: randomized with a minimum value to avoid frozen drops
* Characters: generated from **printable ASCII range (33–126)**

### 5. ANSI Escape Codes

Used to:

* Clear the screen
* Move the cursor
* Set RGB colors

### 6. Rendering Loop

* Draw head + tail for each drop
* Apply **green gradient effect**
* Move drops downward based on speed
* Reset drops when they exit the screen

---

## 🎨 ANSI Escape Codes Reference

Useful resource for terminal control:
👉 [ANSI Escape Sequences](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797)

---

## 💡 Customization Ideas

* Use only `0` and `1` characters for a classic Matrix style
* Change color schemes (e.g., blue, red, cyberpunk themes)
* Add fading effects or motion blur
* Control speed interactively

---

## 🧠 What You’ll Learn

* Terminal graphics programming
* ANSI escape codes and cursor control
* Real-time animation logic
* Randomization techniques
* Memory management (in low-level languages like C)

---

## 🤝 Contributing

Feel free to:

* Add new language implementations
* Improve performance
* Add new visual effects

Pull requests are welcome!

---

## ⭐ Support

If you found this project useful or interesting, consider giving it a ⭐

---

## 📜 License

This project is open-source and available under the MIT License.
