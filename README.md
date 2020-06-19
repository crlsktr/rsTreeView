# Tree Parser
Takes a tree in with the following format:

| Path  | Id | Depth | Class |
|-------|----|-------|-------|
| /1/   | 1  | 0     | 10    |
| /1/2/ | 2  | 1     | 20    |
| /1/3/ | 3  | 1     | 20    |

And turns it into:

```js
{
	id:1,
	class:10,
	children: [
		  {
			id:2,
			class:20,
			children:[]
		  },
		  {
			id:3,
			class:20,
			children:[]
		  }
	]
}
```