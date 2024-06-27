# Data Types

## Primitives

### `string`

### `int`

### `number`

### `boolean`

### `component`

### `tag`

### `arr`

### `vec`

## Custom types

### Create types with comptime function

```guiql
comptime func Vec

```guiql
comptime func Result(T: type, E: type) {
    -> enum {
        Ok(T),
        Err(E),
    }
}
```

### Define types with `type` statement

```guiql
type User {
    id: string key,
    email: string,
    name: string,
}
```

### Tuple

```guiql
const Vec2: type = (number, number)
```

```guiql
let tuple: (int, string) = (1, "hi")
```

### Enumerate

Enumerates can be defined with `enum` statement.

```guiql
enum TaskState {
    Planned,
    Working,
    Done,
}
```
