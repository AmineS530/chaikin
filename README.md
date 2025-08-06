# Chaikin's Algorithm Visualization

This Rust application uses **Chaikin's corner-cutting algorithm** to generate smooth curves from user-defined control points. It is built with the [Macroquad](https://github.com/not-fl3/macroquad) game framework for easy window creation, rendering, and input handling, This is the second raid group Project during our Rust Pool in Zone01Oujda.

## Features

- Draw control points with the left mouse button.
- Drag existing points in real time (even during animation; the curve and subdivision steps recalculate dynamically on drag).
- Press **Enter** to start animation:
  - If fewer than 2 points: displays a warning message and no animation.
  - If 2 or more points: first shows the original connected points, then the 7 Chaikin subdivision steps.
- Press **C** to clear all points and reset.
- Press **Escape** to exit the application.
- Special handling:
  - **<2 points**: pressing Enter shows a warning; points remain visible but no animation.
  - **2 points**: draws a straight line between them after pressing Enter.

## Setup & Run

1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone or download this repo.
3. Navigate to the project folder:
   ```bash
   cd chaikin
   ```
4. Build and run:
  ```bash
  cargo run --release
  ```

## Makefile Usage

You can also use the provided Makefile to simplify common tasks:

```bash
make build    # compile the project in release mode
make run      # build and run the project in release mode
make clean    # remove build artifacts
```

## Chaikin's Algorithm Steps

Chaikin's algorithm is an iterative corner-cutting technique to smooth a polygonal chain:

1. **Start** with an initial list of control points: P0, P1, ..., Pn.
2. **Keep** the first endpoint P0 unchanged.
3. **For each edge** from Pi to Pi+1:
   - Compute a point Q at 75% of the way from Pi to Pi+1:
     \[ Q = 0.75 \cdot P_i + 0.25 \cdot P_{i+1} \]
   - Compute a point R at 25% of the way from Pi to Pi+1:
     \[ R = 0.25 \cdot P_i + 0.75 \cdot P_{i+1} \]
   - Append Q and R to the new point list.
4. **Keep** the last endpoint Pn unchanged.
5. **Repeat** the above subdivision steps for 7 iterations, generating a sequence of increasingly smooth point sets.
6. **Visualize** each iteration as an animation frame.

## Code Highlights

- **`chaikin` function**: generates one subdivision of the point list.
- **Event loop** in `main`: handles input, toggles animation, and renders each frame.
- Real-time dragging uses proximity detection (radius = 10px) and updates the selected point on mouse drag events.

## Dependencies

- `macroquad = "0.3"`

## Author 

* By Ismail Bentour & Abdeladim Jabbouri & Amine Sadik During our Rust Pool in Zone01Oujda.
