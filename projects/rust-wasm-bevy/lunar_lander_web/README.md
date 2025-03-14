# 🚀 Lunar Lander in Rust & Bevy (WebAssembly)

## Plan

1. Project Setup (Rust + Bevy + WASM)
2. Render a simple window
3. Add a basic lander sprite
4. Implement gravity
5. Add controls (left, right, thrust)
6. Add terrain & landing mechanics
7. Polish with UI & sound
8. Deploy the WebAssembly app

## Step 1: Project Setup

We’ll set up a Rust Bevy project targeting WebAssembly (WASM).

### 1.1 Install Dependencies

Before we start, ensure you have the required tools installed:

```bash
    // Rust (latest stable)
    rustup update
    // WebAssembly target support
    rustup target add wasm32-unknown-unknown
    // Trunk (for building & serving WASM apps)
    cargo install trunk
    // Trunk allows us to easily compile and serve our WebAssembly application.
```

### 1.2 Create a New Rust Project

Run the following commands to initialize the project:

```bash
  cargo new lunar_lander_web
  cd lunar_lander_web
```

### 1.3 Add Bevy and WebAssembly Support

Edit Cargo.toml and add Bevy as a dependency:

```bash
  [dependencies]
  bevy = "0.13"
```

Bevy supports WebAssembly out of the box, so no additional dependencies are required.

### 1.4 Create an HTML File for WebAssembly

Create a new file named index.html in the project root:

```html
<!DOCTYPE html>
  <html lang="en">
  <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Lunar Lander</title>
  </head>
  <body>
      <script type="module">
          import init from "./lunar_lander_web.js";
          init();
      </script>
  </body>
  </html>
```

### 1.5 Create a Minimal Bevy App

Now, edit src/main.rs and set up a simple Bevy application:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Load default Bevy plugins (window, rendering, input, etc.)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());
}
```

This initializes Bevy with a 2D camera, preparing the scene for rendering.

### 1.6 Build and Run

Now, let's test the setup by compiling and running it in the browser.
Run in the Browser

```bash
trunk serve --open
```

This will:

- Compile our Bevy app to WebAssembly
- Start a local server at <http://127.0.0.1:8080/>
- Open the game in your browser

#### If error related to: UUID

- 1️⃣ Add the Correct uuid Features for WASM

Edit your Cargo.toml file and add:

```bash
[dependencies]
bevy = { version = "0.13", features = ["dynamic_linking"] }
uuid = { version = "1.6", features = ["js"] }
````

This does two things:

- Adds the js feature to uuid, allowing it to use JavaScript’s built-in random number generator (crypto.getRandomValues()).
- Enables dynamic_linking for Bevy, which improves WASM compatibility.

- 2️⃣ Clean and Rebuild the Project

Now, clean the project and try rebuilding:

```
  cargo clean
     Removed 2264 files, 1.3GiB total
  trunk serve --open
```

This should solve the issue and allow the game to run in the browser.

- 🚀 Why Does This Work?

- The js feature tells uuid to use JavaScript’s randomness.
- Bevy works better with WebAssembly when dynamic_linking is enabled.
- Clearing old builds ensures the fixed settings are applied.

### 1.7 First Commit

Now, let's save our progress in Git:

```bash
cd rust-explorations/projects/rust-wasm-bevy
git add lunar_lander_web
git commit -m "Setup Rust + Bevy + WASM project"
````

- Next Steps

Now that the basic setup is complete, the next steps are:

- Rendering a background (moon surface)
- Adding the Lunar Lander sprite
- Implementing physics (gravity, thrust)
- Adding keyboard controls
