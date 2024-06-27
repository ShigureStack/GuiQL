# GuiQL Language Specification

## Overview

GuiQL is a query language for building graphical user interfaces.

## Example: Define components

```guiql
// Define new component

/*
 * This is a comment.
 */
component TodoItem {
    // Properties
    id: string key,
    name: string,
    isDone: boolean = false,

    // Child components
    HStack {
        styles: {
            padding: (16px, 8px),
            gap: 5px,
        },

        CheckBox isDone {
            styles: {
                size: 18px,
            },
        }
        Label name {
            styles: {
                fontSize: 16px,
            },
        }
    }
}

component TodoList {
    VStack {
        styles: {
            gap: 5px,
        },

        // Child components will be added here.
        Slot {
            children.[
                TodoItem
            ].styles: {
                border: (solid, 1px, black),
            },
        }
    }
}
```

## Examples: Query components

### Create a component

```
@root += create TodoList#todoList {}
```

### Add children to a component

```
@root/todoList += create TodoItem { id = "item0", name = "Todo" }
```

### Find a component with a query

```
@root/todoList.[TodoItem with { id = "item0" }].isDone = true
```

### Delete a component

```
delete @root/todoList.[TodoItem with {id = "item0" }]
```
