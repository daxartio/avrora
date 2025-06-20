import io
import json

import avro.datafile
import avro.io
import avro.schema
import pytest

from avrora._avrora import Avro


@pytest.fixture()
def raw_schema(schema):
    return json.dumps(schema)


@pytest.fixture()
def schema():
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
def example():
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
def expected():
    return [
        {
            "null_field": None,
            "boolean_field": True,
            "int_field": 42,
            "long_field": 1234567890,
            "float_field": 3.140000104904175,
            "double_field": 2.71828,
            "string_field": "Hello, Avro!",
            # fmt: off
            "bytes_field": b'SGVsbG8sIEF2cm8h',
            # fmt: on
            "array_field": [1, 2, 3, 4, 5],
            "map_field": {"key2": "value2", "key1": "value1"},
            "enum_field": "GREEN",
            "fixed_field": b"abcd",
            "union_field": "string value",
            "record_field": {"street": "123 Main St", "city": "Exampleville"},
        },
    ]


def test_avro(raw_schema, example, expected):
    avro_schema = avro.schema.parse(raw_schema)
    buffer = io.BytesIO()

    writer = avro.datafile.DataFileWriter(buffer, avro.io.DatumWriter(), avro_schema)
    writer.append(example)
    writer.flush()

    avrora = Avro.with_schema(raw_schema)
    result = avrora.parse(buffer.getvalue())
    buffer.close()

    assert result == expected
