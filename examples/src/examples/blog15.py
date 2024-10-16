from typing import Generator


def square() -> Generator[float, float | None, None]:
    while True:
        x = yield
        yield x**2


def wrapper(gen: Generator[float, float | None, None]) -> None:
    next(gen)
    while True:
        try:
            x = yield
            gen.send(x)
        except StopIteration:
            pass
