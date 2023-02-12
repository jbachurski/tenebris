# Memorynth

## Description

Fight your way out of a maze you (and the game) can't remember. You can only remember the tiles that you can see around you; if you move away from an area and move back, you may notice that the passageways have.

Place down torches (by pressing E) to remember a small area around the torch. Watch out though, as the passageway leading to the torch could also be forgotten.

This game was written in Rust and using the [Bevy](https://bevyengine.org/) game engine. There are also a couple of experiments written in Python that you can check out in the `demos` directory.

## Development

### Pre-commit hooks

Make sure you have Python installed and it meets `requirements.txt` (`pre-commit`). Use:

```
pre-commit install
```

To install the pre-commit hooks to git.

You can use:
```
pre-commit run --all-files
```

To run the pre-commit hooks manually.
