import io
from timeit import timeit

from avro.datafile import DataFileReader
from avro.io import DatumReader
from fastavro import reader

from avrora._avrora import parse

try:
    import plotly.graph_objects as go
except ImportError:
    go = None

raw_schema = """
{
    "type": "record",
    "name": "test",
    "fields": [
        {"name": "a", "type": "long", "default": 42},
        {"name": "b", "type": "string"}
    ]
}
"""
# fmt: off
data = bytes([
    79, 98, 106, 1, 4, 22, 97, 118, 114, 111, 46, 115, 99, 104, 101, 109, 97, 222, 1, 123,
    34, 116, 121, 112, 101, 34, 58, 34, 114, 101, 99, 111, 114, 100, 34, 44, 34, 110, 97,
    109, 101, 34, 58, 34, 116, 101, 115, 116, 34, 44, 34, 102, 105, 101, 108, 100, 115, 34,
    58, 91, 123, 34, 110, 97, 109, 101, 34, 58, 34, 97, 34, 44, 34, 116, 121, 112, 101, 34,
    58, 34, 108, 111, 110, 103, 34, 44, 34, 100, 101, 102, 97, 117, 108, 116, 34, 58, 52,
    50, 125, 44, 123, 34, 110, 97, 109, 101, 34, 58, 34, 98, 34, 44, 34, 116, 121, 112,
    101, 34, 58, 34, 115, 116, 114, 105, 110, 103, 34, 125, 93, 125, 20, 97, 118, 114, 111,
    46, 99, 111, 100, 101, 99, 14, 100, 101, 102, 108, 97, 116, 101, 0, 4, 9, 246, 238,
    211, 144, 123, 129, 116, 201, 133, 2, 99, 244, 227, 0, 2, 36, 5, 192, 33, 1, 0, 0, 0,
    131, 48, 247, 72, 143, 132, 165, 191, 99, 31, 26, 4, 9, 246, 238, 211, 144, 123, 129,
    116, 201, 133, 2, 99, 244, 227, 0,
])
# fmt: on

def avro_parse(d):
    avro_reader = DataFileReader(io.BytesIO(d), DatumReader())
    return next(avro_reader)

def fastavro_parse(d):
    avro_reader = reader(io.BytesIO(d))
    return next(avro_reader)

number = 1000

results = [
    timeit(lambda: avro_parse(data), number=number),
    timeit(lambda: parse(data), number=number),
    timeit(lambda: fastavro_parse(data), number=number),
]

print(results)
