# Tools for Intersystem Communication

Almost none of your systems will work in isolation. Instead, you'll want to pass data back and forth between them as your game progresses, altering the game state and advancing .

Bevy offers a wide array of tools you could use to do this. Here's a full list (in order of ascending complexity) and a one-sentence description of each of them, most of which are explored later in this section:

1. Modify the same [components](../components.md): this is your bread-and-butter for data that lives with a specific entity.
2. Modify the same [resource](resources.md): this is the standard pattern for changing or reading persistent data that isn't attached to a specific entity.
3. Modify [commands](commands.md): this is used to accumulate changes that affect diverse data (like adding entities) until the end of the stage.
4. Call a second system from the first as a function like any other in Rust: can be handy if you want to repeat functionality.
5. Send [events](events.md) from one system to another: great for open-ended message passing that can persist between ticks.
6. [Chain](chaining.md) one system directly into another while passing some data: think of this like currying or a decorator.

While you *can* probably coerce most of these approaches to perform another's role, picking the wrong tool will make your code harder to read, harder to maintain, and probably less performant. Skim each of the sections linked to get a brief overview, then come back to them in detail when you need to implement that approach.
