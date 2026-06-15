# My Jira CLI

This is a simple [Jira](https://www.atlassian.com/software/jira)-like project management CLI app built in Rust.

## What Can This App Do

This app models 2 entities:

- `Epic`, i.e. project
- `Story`, i.e. task

An `Epic` may contain 0 or more `Stories`.

In this app, you can perform CRUD operations on Epic and Story entities, and the data is persisted as a JSON file.

## Developer Notes

### Entity Relation Diagram

![Entity Relation Diagram](./doc/entity_relation_diagram.png)

### Things to Improve

- For menu navigation, make the app read 1 char and immediately process it, instead of waiting for user to enter a string and then still need to press ENTER key.
