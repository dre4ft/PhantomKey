# **PhantomKey**  

Unleash the full potential of your HID devices with **PhantomKey**—a next-generation scripting engine designed for precision, flexibility, and ease of use. PhantomKey takes HID automation to the next level, integrating scripting, event handling, and seamless device interaction into a single, powerful tool.  

---

## **🚀 What is PhantomKey?**  
PhantomKey is a unified HID scripting **engine** that enables the creation and execution of complex workflows for keyboards, mice, and other HID devices.  
From parsing scripts to streaming HID commands directly to your devices, PhantomKey combines everything into one cohesive process, giving you complete control over multiple interfaces in real time.  

With PhantomKey, you can script, execute, and manage intricate HID events with unparalleled simplicity and power.  

---

## **✨ Features**  
- **Unified Execution**: Compile, stream, and manage HID events in a single workflow.  
- **Multi-Interface Support**: Simultaneously control multiple HID interfaces (keyboard + mouse).  
- **Event Handling**: Execute advanced actions like `WAIT` for dynamic, context-aware automation.  
- **Custom HID Actions**: Keyboard input, mouse movement, delays, and more.  
- **Real-Time Streaming**: Send HID commands directly to devices without intermediate steps.  
- **Cross-Platform Compatibility**: Works on Linux, macOS, and Windows.  
- **Powerful Syntax**: Write concise, human-readable scripts for maximum efficiency.  

---

## **🛠️ How it Works**  
1. **Write Your Script**  
   Use PhantomKey's simple and flexible syntax to define your automation flow:  
   ```plaintext
   DELAY 1000
   STRING Hello, World!
   ENTER
   WAIT "USB Connected"
   MOVE_MOUSE 100, 50
   CLICK LEFT
   ```

2. **Execute Your Script**  
   PhantomKey handles the entire lifecycle of your script: parsing, compiling, and sending HID events to your chosen interfaces.  
   ```bash
   phantomkey -run my_script.pk -interface keyboard:1 mouse:2
   ```

3. **Control Multiple Devices**  
   Stream commands to multiple HID interfaces in parallel for advanced use cases.  

4. **Manage Events**  
   Use context-sensitive events like `WAIT` to pause execution based on real-world conditions.  

---

## **📦 Installation**  
Clone the repository and build PhantomKey:  
```bash
git clone https://github.com/yourusername/phantomkey.git
cd phantomkey/src/
cargo build --release
```

---

## **📝 Syntax Overview**  
PhantomKey’s syntax is designed for clarity and power, supporting both simple and advanced workflows.  

| **Command**         | **Description**                                      |  
|----------------------|------------------------------------------------------|  
| `STRING <text>`      | Types the provided text.                             |  
| `DELAY <ms>`         | Waits for the specified number of milliseconds.      |  
| `WAIT <condition>`   | Pauses execution until the specified condition is met. |  
| `MOVE_MOUSE <x, y>`  | Moves the mouse by the given x and y deltas.         |  
| `CLICK <button>`     | Clicks a mouse button (`LEFT`, `RIGHT`, `MIDDLE`).   |  
| `ENTER`              | Simulates the "Enter" key press.                    |  

---

## **🌌 Why PhantomKey?**  
PhantomKey is the ultimate tool for HID automation enthusiasts, tinkerers, and professionals. Its real-time capabilities, multi-interface support, and event-driven design make it an indispensable companion for debugging, workflow automation, or experimentation.  

---

## **🌟 Roadmap**  
- [x] Unified HID scripting engine.  
- [x] Real-time streaming to multiple interfaces.  
- [x] Event-driven commands like `WAIT`.  
- [x] Basic script parsing and execution.  
- [ ] Advanced HID command compiler for `.bin` generation.  
- [ ] Support for conditional scripting (e.g., `IF/ELSE`).  
- [ ] Enhanced device diagnostics and state tracking.  
- [ ] Extensible plugin system for custom commands and events.  
- [ ] Integrated debugging tools for testing and validation.  

---

## **👩‍💻 Contributing**  
We’re building PhantomKey for everyone. Contribute by reporting bugs, suggesting features, or submitting pull requests.  

---

## **⚖️ License**  
PhantomKey is open-source and licensed under the [MIT License](LICENSE).  
