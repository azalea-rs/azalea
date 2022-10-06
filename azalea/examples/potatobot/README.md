A relatively complex bot for farming potatoes.

Note: At the moment, all of the code here is only hypothetical. I decided to write this to help me decide how I want some the APIs to look.

## Attempted
- Sync: a sync function is called with the state and bot every time we get an event, and the function can queue events to execute at the end of the tick

    Pros: No .lock().unwrap() necessary, and theoretically pausable by saving the state.

    Cons: Async functions like opening containers and pathfinding are annoying because you have to keep state for them, and the code generally ends up being more confusing.

- Async non-blocking: an async function is called in a new task with the state mutex and bot every time we get an event

    Pros: Easier to do async stuff like interacting with containers, code is somewhat easier to understand

    Cons: Lock spam everywhere is annoying, and you have to make sure stuff doesn't accidentally run in parallel.

## Considered:
(I didn't actually try this because the problems were apparent)
- Async blocking: an async function is called with the state and bot every time we get an event, and only handles the next event when this one finishes running

    Pros: No lock spam

    Cons: Sometimes you want to handle multiple events at once like eating if you get hungry while pathfinding, this makes it harder without increasing complexity
