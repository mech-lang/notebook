<p align="center">
  <img width="500px" src="https://mech-lang.org/img/logo.png">
</p>

Mech is a language for developing **data-driven**, **reactive** systems like animations, games, and robots. It makes **composing**, **transforming**, and **distributing** data easy, allowing you to focus on the essential complexity of your problem. 

Read about progress on our [blog](https://mech-lang.org/blog/), follow us on Twitter [@MechLang](https://twitter.com/MechLang), or join the mailing list: [talk@mech-lang.org](https://mech-lang.org/page/community/).

# Mech Notebook

A graphical interface for Mech using a notebook metaphor. Mech notebook presents the world as a grid of cells, into which you can log data, write queries over that data, collect and write notes, draw interfaces, and design visualizations.

*Note: Right now the editor is a text area and a region that draws program output, but this will eventually evolve into the above vision.*

## Building and Running

Download and install [NodeJS](https://nodejs.org) and then run

```bash
> npm install
```

You can start a file server that will host the editor with

```bash
> npm start
```

Then navigate a browser to `localhost:8080`. A packaged version can be built with

```bash
> npm run build
```

## Project Status

Mech is currently in the **alpha** stage of development. This means that while some features work and are tested, programs are still likely to crash and produce incorrect results. We've implemented some language features, but many are not yet implemented.

Feel free to use the language for your own satisfaction, but please don't use it for anything important.

## License

Apache 2.0