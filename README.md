# avrora

<!-- [![PyPI](https://img.shields.io/pypi/v/avrora)](https://pypi.org/project/avrora/) -->
<!-- [![PyPI - Python Version](https://img.shields.io/pypi/pyversions/avrora)](https://www.python.org/downloads/) -->
<!-- [![GitHub last commit](https://img.shields.io/github/last-commit/daxartio/avrora)](https://github.com/daxartio/avrora) -->
<!-- [![GitHub stars](https://img.shields.io/github/stars/daxartio/avrora?style=social)](https://github.com/daxartio/avrora) -->

## Installation

WIP

```
pip install avrora
```

## Benches

```
---------------------------------------------------------------------------------------- benchmark: 5 tests ----------------------------------------------------------------------------------------
Name (time in us)                 Min                 Max               Mean             StdDev             Median               IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_avrora                   11.1600 (1.0)      118.4300 (1.0)      11.8730 (1.0)       1.9263 (1.0)      11.7200 (1.0)      0.3880 (1.48)       74;156       84.2245 (1.0)       14970           1
test_fastavro                 15.7240 (1.41)     141.0980 (1.19)     16.7499 (1.41)      4.5344 (2.35)     16.2500 (1.39)     0.2620 (1.0)       129;340       59.7019 (0.71)       8161           1
test_avrora_with_schema       24.7390 (2.22)     171.8080 (1.45)     26.3057 (2.22)      6.1312 (3.18)     25.5690 (2.18)     0.7340 (2.80)      136;278       38.0145 (0.45)       7722           1
test_fastavro_with_schema     25.8090 (2.31)     363.2970 (3.07)     27.9061 (2.35)      6.9890 (3.63)     27.2060 (2.32)     0.7930 (3.03)      286;605       35.8345 (0.43)      15605           1
test_avro                     67.9160 (6.09)     294.3420 (2.49)     72.2701 (6.09)     11.4160 (5.93)     70.1290 (5.98)     1.7280 (6.60)      139;318       13.8370 (0.16)       4204           1
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
```

## License

* [MIT LICENSE](LICENSE)

## Contribution

[Contribution guidelines for this project](CONTRIBUTING.md)
