# Chaikin's Corner Cutting Algorithm

An interactive visualization of Chaikin's corner cutting algorithm - a simple yet elegant method for creating smooth curves from straight line segments.

## Installation

### 1. Install Rust (Rootless Rustup)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh
```

### 2. Add Dependencies

Dependencies like `macroquad` are already in `Cargo.toml`, they'll install automatically.

## How to Run

```bash
cargo r
```

## How It Works

1. Click on the canvas to create control points (minimum 2 required)
2. Press Enter to start the animation
3. Watch as the algorithm applies Chaikin's corner cutting iteratively
4. The visualization shows all 7 iterations cycling smoothly
5. Press C to clear and draw a new shape
