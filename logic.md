
## **PhantomKey Keywords and Parameters**

| **Keyword**         | **Inputs**                         | **Description**                                                                 |
|----------------------|-------------------------------------|---------------------------------------------------------------------------------|
| `STRING <text>`      | `<text>` (string)                  | Types the provided text exactly as written.                                    |
| `DELAY <ms>`         | `<ms>` (integer, milliseconds)     | Pauses execution for the specified duration in milliseconds.                   |
| `MOVE_MOUSE <x, y>`  | `<x>` (integer), `<y>` (integer)   | Moves the mouse cursor by the specified x and y deltas.                        |
| `CLICK <button>`     | `<button>` (LEFT, RIGHT, MIDDLE)   | Simulates a mouse button click.                                                |
| `ENTER`              | None                               | Simulates the "Enter" key press.                                               |
| `BACKSPACE`          | None                               | Simulates the "Backspace" key press.                                           |
| `TAB`                | None                               | Simulates the "Tab" key press.                                                 |
| `ESCAPE`             | None                               | Simulates the "Escape" key press.                                              |
| `ARROW <direction>`  | `<direction>` (UP, DOWN, LEFT, RIGHT) | Simulates an arrow key press in the specified direction.                       |
| `HOLD <key>`         | `<key>` (e.g., SHIFT, CTRL)        | Presses and holds the specified key.                                           |
| `RELEASE <key>`      | `<key>` (e.g., SHIFT, CTRL)        | Releases the specified key.                                                    |
| `TYPE <keys>`        | `<keys>` (string, special chars)   | Types a sequence of keys, including modifiers (e.g., `CTRL+ALT+DEL`).          |
| `SCROLL <amount>`    | `<amount>` (integer, positive/negative) | Scrolls the mouse wheel by the specified amount (negative for reverse).       |

---

## **Keyword Behavior Notes**

### **Basic Input Keywords**
1. **`STRING <text>`**  
   - Example: `STRING Hello, World!`  
   - Behavior: Converts the text into HID key presses for each character.

2. **`DELAY <ms>`**  
   - Example: `DELAY 1000`  
   - Behavior: Pauses execution for 1 second (1000 ms).

---

### **Mouse Control**
3. **`MOVE_MOUSE <x, y>`**  
   - Example: `MOVE_MOUSE 100, -50`  
   - Behavior: Moves the cursor right by 100 pixels and up by 50 pixels.

4. **`CLICK <button>`**  
   - Example: `CLICK LEFT`  
   - Behavior: Simulates a left mouse click.

5. **`SCROLL <amount>`**  
   - Example: `SCROLL -3`  
   - Behavior: Scrolls down by 3 units.

---

### **Key Actions**
6. **`ENTER`**  
   - Simulates pressing the "Enter" key.

7. **`BACKSPACE`**  
   - Simulates pressing the "Backspace" key.

8. **`TAB`**  
   - Simulates pressing the "Tab" key.

9. **`ESCAPE`**  
   - Simulates pressing the "Escape" key.

10. **`HOLD <key>` / `RELEASE <key>`**  
    - Example:  
      ```plaintext
      HOLD SHIFT
      STRING hello
      RELEASE SHIFT
      ```
    - Behavior: Types "HELLO".

11. **`ARROW <direction>`**  
    - Example: `ARROW UP`  
    - Behavior: Simulates pressing the "Up" arrow key.

12. **`TYPE <keys>`**  
    - Example: `TYPE CTRL+ALT+DEL`  
    - Behavior: Sends a combination of keys.

---
