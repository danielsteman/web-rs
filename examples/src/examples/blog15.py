from typing import Generator
import asyncio


def square() -> Generator:
    while True:
        x = yield
        if x is not None:
            yield x**2


def wrapper(gen: Generator) -> Generator:
    next(gen)
    while True:
        try:
            x = yield
            gen.send(x)
        except StopIteration:
            pass


def wrapper2(gen: Generator) -> Generator:
    next(gen)
    while True:
        try:
            x = yield
            gen.send(x)
        except StopIteration:
            pass


async def async_square(x: float) -> float:
    return x**2


async def async_wrapper(x: float) -> float:
    return await async_square(x)


if __name__ == "__main__":
    wrapper(square())
    asyncio.run(async_wrapper(2))
