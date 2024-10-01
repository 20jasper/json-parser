Only handles non nested json object with no keys

| state  | {      | }     | EOF   | Other |
| ------ | ------ | ----- | ----- | ----- |
| Init   | Object | Error | Error | Error |
| Object | Error  | End   | Error | Error |
| End    | Error  | Error | Ok    | Error |
