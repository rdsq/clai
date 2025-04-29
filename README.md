# clai

A custom **CLIent** *(get it? CLI and client how funny)* for different AI things. Primarily **chatting**, but also some **embedding** functionality

## Interfaces

The format this thing uses for naming models is **interfaces**. It is `provider:model`. Available providers:

- `ollama`

- `google` *(set the `GOOGLE_GENAI_API_KEY` env var)*

## Installation

Since it is a **Rust** project, it is pretty straightforward

Clone this repository wherever you like, and run

```sh
cargo install --path .
```

*(this might take a while)*

## Chatting

This is the main functionality of this project, *but not the only one*

### Generate

Simlest one of them, simply **generate** a response from a prompt. So it's something like

```sh
clai gen ollama:gemma3:1b "Hello World"
```

### Chat

This is exactly what you would expect. Open a chat with a chatbot

```sh
clai chat ollama:gemma3:1b
```

Chat mode also has **commands**! Type `/help` in chat to see more

### Read (and saves in general)

It also has a feature where you can define an autosave file in `gen` and `chat` using the `--file` option

To read these JSON files, you can use this command

```sh
clai read ./chat.json
```

### Model With Model

This is rather for fun, make two AIs talk to one another

You can define them to be the same model, different ones, whatever sounds fun

```sh
clai model-with-model "Hello World" ollama:gemma3:1b google:gemini-2.0-flash
```

## Embeddings

These are not about chatbots, they're about **embeddings**

*Basically a thing that evaluates the semantic meaning of strings*

### Semantic Search

So basically, you can do stuff like

```sh
clai semsearch ollama:nomic-embed-text 'fruit' 'apple' 'strawberry' 'banana'
```

Or with **files**

```sh
clai semsearch ollama:nomic-embed-text 'nice weather' --input-format file my-posts/*
```

Or with **JSON** *(which is for the next feature)*

### Embed

This one is useful as caching **embeddings** for the `semsearch` command. So:

```sh
clai embed ollama:nomic-embed-text 'The weather is nice today' 'Cats are awesome' 'Rust is cool' 'hello world' --output-format json > my-posts.json
```

And then:

```sh
cat my-posts.json | clai semsearch ollama:nomic-embed-text -f json 'cats'
```

## Planned features

- More interfaces *(not a priority for me personally since I don't use other APIs right now)*

- Images and multimodality

- Option to disable streaming (for better formatting)
