"""
Имплементации разных JSON парсеров
"""
import json
import typing

from vcore.pluginable.json_parser.base import JSONParserBase

try:
    import orjson
except ImportError:  # pragma: no cover
    orjson = None


try:
    import ujson
except ImportError:  # pragma: no cover
    ujson = None


class BuiltInJSONParser(JSONParserBase):
    @staticmethod
    def dumps(data: typing.Dict[str, typing.Any]) -> typing.Union[str, bytes]:
        return json.dumps(data, ensure_ascii=False, separators=(",", ":"))

    @staticmethod
    def loads(string: typing.Union[str, bytes]) -> typing.Any:
        return json.loads(string)


class ORJSONParser(JSONParserBase):
    @staticmethod
    def dumps(data: typing.Dict[str, typing.Any]) -> typing.Union[str, bytes]:
        return orjson.dumps(data)  # pragma: no cover

    @staticmethod
    def loads(string: typing.Union[str, bytes]) -> typing.Any:
        return orjson.loads(string)  # pragma: no cover


class UJSONParser(JSONParserBase):
    @staticmethod
    def dumps(data: typing.Dict[str, typing.Any]) -> typing.Union[str, bytes]:
        return ujson.dumps(data, ensure_ascii=False)  # pragma: no cover

    @staticmethod
    def loads(string: typing.Union[str, bytes]) -> typing.Any:
        return ujson.loads(string)  # pragma: no cover


json_parser_policy: JSONParserBase
"""
`json_parser_policy` -- установленный JSON парсер, используемый по умолчанию
"""

if orjson is not None:
    json_parser_policy = ORJSONParser()
elif ujson is not None:
    json_parser_policy = UJSONParser()
else:
    json_parser_policy = BuiltInJSONParser()
