from __future__ import annotations

import abc
import contextlib
import dataclasses
import typing

from vcore.pluginable.json_parser.base import JSONParserBase


@dataclasses.dataclass
class BaseResponse(abc.ABC):
    @property
    @abc.abstractmethod
    def headers(self) -> typing.Mapping:
        pass

    @abc.abstractmethod
    async def parse_body_json(
        self, json_parser: typing.Optional[JSONParserBase] = None
    ) -> dict:
        pass


@dataclasses.dataclass
class BaseConnector(abc.ABC):
    @classmethod
    @contextlib.asynccontextmanager
    @abc.abstractmethod
    def new(cls) -> typing.AsyncContextManager[BaseConnector]:
        pass

    @contextlib.asynccontextmanager
    @abc.abstractmethod
    async def post_form_encoded_data(self, url: str, data: dict):
        pass
