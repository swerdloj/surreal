# Paper Goals
- Research paper
  - Possible Titles:
    - "Declarative User Interfaces: Advantages and Architecture"
    - "Surreal: A Declarative User Interface Case Study"
      - Reference original declarative approaches (baseline)

- Focus on declarative style API
  - Argue for this rather than the library itself
  - Use library as a case study
  - Examine backend structure
    - Connect program structure, backend, and declarative style together
  - Mention key aspects
    - Declarative views with hooks for extended functionality
    - Message-driven (Elm architecture)
      - Mention observer pattern as an alternative
    - State persists
      - Thread-safe
      - Accessible by all widgets within view tree
    - View structure
      - State, widgets, components, views
      - Share ViewElement trait
      - Tree is represented by arrays of arrays
        - Operations are recursive

- Compare to existing solutions (like GTK & QT) and demonstrate advantages of declarative style over imperative

- Explain how I reached my conclusions

- Also point out downsides of declarative approach

- Relate to languages beyond Rust

- Argue in favor of using existing solutions, but declarative wrappers

- Look at Surreal as an API rather than a library







# Extended Goals
1. Procedural Macro for View Creation (custom syntax)
2. QML-style markup language
3. Boostrapped Designer (like Android Studio)



# Goals
1. Ease of use
  - See what people will do with the library
  - Make those features easy to use/simple
2. Performance
3. Customizability


# White Paper
- Can do in HTML
  - Website format (look into GitHub sites)
  - Look into format like https://bevyengine.org/community/
    - https://pages.github.com/
    - Can use custom domain at some point
  - Should show future potential
    - Encourage contributions
- First build basic feature set
- Consider video companion

## Purpose
- Introduce the project
  - What it solves
  - What it does
  - Why this over alternatives
- Specify intentions
- Relate to community (Rust & GUI development)
- Cross-platform & future-proof

## Implementation
- Explain how it accomplishes its various goals
- How it can/will grow


## Features
- Track animation state
  - Stop rendering when no new frames are needed


## Benchmarking
- Time per frame
- Note performance
  - Compare to Rust libraries
    - Bevy, Conrad, Iced, etc.
  - Compare to existing solution
    - Unity, Electron, etc.
  - Integrated GPU vs discrete
- CPU vs GPU frame times
- Compare to software rendering
  - Low powered devices such as Raspberry Pi


# Documentation

## Examples
- Provide examples of increasing complexity
- Provide real-world examples
  - Implement UIs of Discord, Spotify, etc.
  - Create an actual application
    - Notepad, Music Player, Calculator, etc.
- Focus on ability to learn from the examples

## Library Documentation
- Usual docs.rs documentation
- GitHub readme