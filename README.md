# **PhantomKey**  
A sleek, powerful, and customizable scripting language for creating HID-based automation. Transform your Human Interface Device (HID) into a precision tool with PhantomKey—crafted for flexibility, ease of use, and lightning-fast execution.

---

## **🚀 What is PhantomKey?**  
PhantomKey is a custom scripting language designed to make writing HID scripts simple yet powerful. Whether you're building macros, automating workflows, or experimenting with HID devices, PhantomKey compiles your scripts into `.bin` files ready to be flashed onto a device like a USB Rubber Ducky or any compatible programmable HID.

With PhantomKey, you're in control of every keystroke, mouse movement, and action your device executes.

---

## **✨ Features**  
- **Human-Readable Syntax**: Intuitive and straightforward commands for rapid scripting.  
- **Custom HID Actions**: Support for keyboard input, mouse movement, delays, and more.  
- **Fast Compilation**: Turn your scripts into `.bin` files in seconds.  
- **Cross-Platform**: Use PhantomKey on Linux, macOS, or Windows.  
- **Expandable**: Open-source and customizable—extend the language to fit your needs.  
- **Stealth and Precision**: Perfect for debugging, testing, or creating automation workflows.  

---

## **🛠️ How it Works**  
1. **Write Your Script**  
   Use PhantomKey's simple and flexible syntax. Here's an example:  
   ```plaintext
   DELAY 1000
   STRING Hello, World!
   ENTER
   DELAY 500
   MOVE_MOUSE 50, 50
   CLICK LEFT
   ```

2. **Compile Your Script**  
   Run the PhantomKey compiler to generate a `.bin` file:  
   ```bash
   phantomkey-compiler my_script.pk -o output.bin
   ```

3. **Flash Your Device**  
   Use your favorite HID device flashing tool to load the `.bin` file.

4. **Run and Watch the Magic**  
   Plug in your HID device and see your script in action.

---

## **📦 Installation**  
Clone the repository and build the compiler:  
```bash
git clone https://github.com/yourusername/phantomkey.git
cd phantomkey/src/
cargo build --release  # Or use the precompiled binaries
```

---

## **📝 Syntax Overview**  
PhantomKey's syntax is clean, readable, and powerful.  

| **Command**         | **Description**                                      |  
|----------------------|------------------------------------------------------|  
| `STRING <text>`      | Types the provided text.                             |  
| `DELAY <ms>`         | Waits for the specified number of milliseconds.      |  
| `MOVE_MOUSE <x, y>`  | Moves the mouse by the given x and y deltas.         |  
| `CLICK <button>`     | Clicks a mouse button (`LEFT`, `RIGHT`, `MIDDLE`).   |  
| `ENTER`              | Simulates the "Enter" key press.                    |  


---

## **👩‍💻 Contributing**  
We welcome contributions to make PhantomKey even better! Here's how you can help:  
- Report bugs and suggest features via [issues](issues/).  
- Submit pull requests with new features, bug fixes, or improvements.  
- Share your scripts and use cases to inspire others.  

---

## **⚖️ License**  
PhantomKey is open-source and licensed under the [MIT License](LICENSE).

---

## **🌟 Why Choose PhantomKey?**  
PhantomKey is built to empower developers, tinkerers, and automation enthusiasts. Its simplicity and versatility make it perfect for beginners and experts alike. Whether you’re looking to automate repetitive tasks, experiment with HID devices, or simply learn something new, PhantomKey is your go-to scripting language.

---


