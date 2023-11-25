import io
import json
from typing import Any

import avro.schema
import fastavro
import pytest
from avro.datafile import DataFileReader, DataFileWriter
from avro.io import DatumReader, DatumWriter

import avrora


@pytest.fixture()
def raw_schema(schema: Any) -> str:
    return json.dumps(schema)


@pytest.fixture()
def schema() -> Any:
    return {
        "namespace": "example.avro",
        "type": "record",
        "name": "Example",
        "fields": [
            {"name": "null_field", "type": "null"},
            {"name": "boolean_field", "type": "boolean"},
            {"name": "int_field", "type": "int"},
            {"name": "long_field", "type": "long"},
            {"name": "float_field", "type": "float"},
            {"name": "double_field", "type": "double"},
            {"name": "string_field", "type": "string"},
            {"name": "bytes_field", "type": "bytes"},
            {"name": "array_field", "type": {"type": "array", "items": "int"}},
            {"name": "map_field", "type": {"type": "map", "values": "string"}},
            {
                "name": "enum_field",
                "type": {
                    "type": "enum",
                    "name": "Color",
                    "symbols": ["RED", "GREEN", "BLUE"],
                },
            },
            {"name": "fixed_field", "type": {"type": "fixed", "name": "Id", "size": 4}},
            {"name": "union_field", "type": ["null", "string", "int"]},
            {
                "name": "record_field",
                "type": {
                    "type": "record",
                    "name": "Address",
                    "fields": [
                        {"name": "street", "type": "string"},
                        {"name": "city", "type": "string"},
                    ],
                },
            },
        ],
    }


@pytest.fixture()
def example() -> Any:
    return {
        "null_field": None,
        "boolean_field": True,
        "int_field": 42,
        "long_field": 1234567890,
        "float_field": 3.14,
        "double_field": 2.71828,
        "string_field": "Hello, Avro!",
        "bytes_field": b"SGVsbG8sIEF2cm8h",
        "array_field": [1, 2, 3, 4, 5],
        "map_field": {"key1": "value1", "key2": "value2"},
        "enum_field": "GREEN",
        "fixed_field": b"abcd",
        "union_field": "string value",
        "record_field": {"street": "123 Main St", "city": "Exampleville"},
    }


@pytest.fixture()
def data(example: Any, raw_schema: str) -> bytes:
    avro_schema = avro.schema.parse(raw_schema)
    buffer = io.BytesIO()

    writer = DataFileWriter(buffer, DatumWriter(), avro_schema)
    writer.append(example)
    writer.flush()

    val = buffer.getvalue()
    buffer.close()
    return val


def avrora_parse(parser, d):
    return parser.parse(d)


def avrora_parse_with_schema(parser, d):
    return parser.parse(d)


def avro_parse(d):
    avro_reader = DataFileReader(io.BytesIO(d), DatumReader())
    return next(avro_reader)


def fastavro_parse(d):
    avro_reader = fastavro.reader(io.BytesIO(d))
    return next(avro_reader)


def fastavro_parse_with_schema(d, schema):
    avro_reader = fastavro.reader(io.BytesIO(d), schema)
    return next(avro_reader)


def test_avrora(benchmark: Any, data: bytes) -> None:
    parser = avrora.Avro()
    benchmark(avrora_parse, parser, data)


def test_avrora_with_schema(benchmark: Any, raw_schema: str, data: bytes) -> None:
    parser = avrora.Avro.with_schema(raw_schema)
    benchmark(avrora_parse_with_schema, parser, data)


def test_avro(benchmark: Any, data: bytes) -> None:
    benchmark(avro_parse, data)


def test_fastavro(benchmark: Any, data: bytes) -> None:
    benchmark(fastavro_parse, data)


def test_fastavro_with_schema(benchmark: Any, raw_schema: str, data: bytes) -> None:
    schema = fastavro.parse_schema(json.loads(raw_schema))
    benchmark(fastavro_parse_with_schema, data, schema)
