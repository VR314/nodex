# DX Design Document

How should developers interface with Nodex? The experience should be intuitive and frictionless. Overhead to start a new project should be as little as possible.


Creating a new nodex project should have a friendly dialog to get set up, similar to cookiecutters. There should also be an option to pass in an already-created config file to bypass the prompts.

```
> nodex project new <name> -c <config file>
Enter your project's name:
Enter your first node's name:
...
Created project in <name>!

```

Project structure should be as follows:
```
project_name
  ├── topics
  │   ├── topic_1.topic
  │   └── topic_2.topic
  └── nodes
      ├── node_1
      │   ├── node_1.node
      │   └── node_1.py
      └── node_2
          ├── node_2.node
          └── node_2.js 
```

```
> nodex node new <project-root> <name> -c <config file>
Enter your node's name:
Enter the language you want this node to be written in:
...

Created node in <project-root>/<name>!
```


```
> nodex run <node_name> <node_name_2> --debug
...
Node <node_name> started!
Node <node_name_2> started!
Node <node_name> connected to <topic_name>
Node <node_name_2> connected to <topic_name>
Node <node_name> sent message to <topic_name>: "Hello, world!"
Node <node_name_2> received message from <topic_name>: "Hello, world!"

^C
Node <node_name> stopped!
Node <node_name_2> stopped!
```