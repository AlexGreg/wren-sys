^title Fiber Class
^category core

A lightweight coroutine. [Here](../fibers.html) is a gentle introduction.

### new **Fiber**(function)

Creates a new fiber that executes `function` in a separate coroutine when the
fiber is run. Does not immediately start running the fiber.

    :::dart
    var fiber = new Fiber {
      IO.print("I won't get printed")
    }

## Static Methods

### Fiber.**current**

The currently executing fiber.

### Fiber.**yield**()

Pauses the current fiber and transfers control to the parent fiber. "Parent"
here means the last fiber that was started using `call` and not `run`.

    :::dart
    var fiber = new Fiber {
      IO.print("Before yield")
      Fiber.yield()
      IO.print("After yield")
    }

    fiber.call()            // "Before yield"
    IO.print("After call")  // "After call"
    fiber.call()            // "After yield"

When resumed, the parent fiber's `call()` method returns `null`.

If a yielded fiber is resumed by calling `call()` or `run()` with an argument,
`yield()` returns that value.

    :::dart
    var fiber = new Fiber {
      IO.print(Fiber.yield()) // "value"
    }

    fiber.call()        // Run until the first yield.
    fiber.call("value") // Resume the fiber.

If it was resumed by calling `call()` or `run()` with no argument, it returns
`null`.

If there is no parent fiber to return to, this exits the interpreter. This can
be useful to pause execution until the host application wants to resume it
later.

    :::dart
    Fiber.yield()
    IO.print("this does not get reached")

### Fiber.**yield**(value)

Similar to `Fiber.yield` but provides a value to return to the parent fiber's
`call`.

    :::dart
    var fiber = new Fiber {
      Fiber.yield("value")
    }

    IO.print(fiber.call()) // "value"

## Methods

### **call**()

**TODO**

### **call**(value)

**TODO**

### **isDone**

Whether the fiber's main function has completed and the fiber can no longer be
run. This returns `false` if the fiber is currently running or has yielded.

### **run**()

**TODO**

### **run**(value)

**TODO**
