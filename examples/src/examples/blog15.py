from typing import Generator, Any


def square() -> Generator[None, Any, Any]:
    while True:
        x = yield
        if x is not None:
            yield x**2


def wrapper(gen: Generator[float, float | None, None]) -> None:
    next(gen)
    while True:
        try:
            x = yield
            gen.send(x)
        except StopIteration:
            pass
