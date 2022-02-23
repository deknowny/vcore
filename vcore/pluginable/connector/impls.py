from __future__ import annotations

import contextlib
import dataclasses
import ssl
import typing

import aiohttp
import certifi

from vcore.pluginable.connector.base import BaseConnector, BaseResponse
from vcore.pluginable.json_parser.base import JSONParserBase
from vcore.pluginable.json_parser.impls import json_parser_policy

ssl_context = ssl.create_default_context(cafile=certifi.where())


async def new_aiohttp_session():
    return aiohttp.ClientSession(
        connector=aiohttp.TCPConnector(ssl=ssl_context),
        skip_auto_headers={"User-Agent"},
        raise_for_status=True,
    )


@dataclasses.dataclass
class AIOHTTPResponse(BaseResponse):

    response: aiohttp.ClientResponse

    @property
    def headers(self) -> typing.Mapping:
        return self.response.headers

    async def parse_body_json(
        self, json_parser: typing.Optional[JSONParserBase] = None
    ) -> dict:
        parser = json_parser or json_parser_policy
        return await self.response.json(loads=parser.loads)


@dataclasses.dataclass
class AIOHTTPConnector(BaseConnector):

    session: aiohttp.ClientSession

    @classmethod
    @contextlib.asynccontextmanager
    async def new(cls) -> typing.AsyncContextManager[AIOHTTPConnector]:
        async with await new_aiohttp_session() as session:
            connector = AIOHTTPConnector(session=session)
            yield connector

    @contextlib.asynccontextmanager
    async def post_form_encoded_data(
        self, url: str, data: dict
    ) -> AIOHTTPResponse:
        async with self.session.post(url, data=data) as response:
            yield AIOHTTPResponse(response=response)


connector_policy: typing.Type[BaseConnector] = AIOHTTPConnector
