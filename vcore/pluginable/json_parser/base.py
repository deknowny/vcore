from __future__ import annotations

import abc
import typing


class JSONParserBase(abc.ABC):
    @staticmethod
    @abc.abstractmethod
    def dumps(data: typing.Dict[str, typing.Any]) -> typing.Union[str, bytes]:
        pass

    @staticmethod
    @abc.abstractmethod
    def loads(
        string: typing.Union[str, bytes]
    ) -> typing.Dict[str, typing.Any]:
        pass
