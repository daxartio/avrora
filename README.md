# avrora

<!-- [![PyPI](https://img.shields.io/pypi/v/avrora)](https://pypi.org/project/avrora/) -->
<!-- [![PyPI - Python Version](https://img.shields.io/pypi/pyversions/avrora)](https://www.python.org/downloads/) -->
<!-- [![GitHub last commit](https://img.shields.io/github/last-commit/daxartio/avrora)](https://github.com/daxartio/avrora) -->
<!-- [![GitHub stars](https://img.shields.io/github/stars/daxartio/avrora?style=social)](https://github.com/daxartio/avrora) -->

WIP

## Benches

It is not faster than fastavro so far.

```
------------------------------------------------------------------------------------------ benchmark: 5 tests -----------------------------------------------------------------------------------------
Name (time in us)                  Min                 Max                Mean             StdDev              Median               IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_fastavro                  44.7170 (1.0)      204.2050 (1.0)       49.3465 (1.0)      12.9435 (1.05)      45.7390 (1.0)      0.7925 (1.0)        78;152       20.2648 (1.0)        1016           1
test_avrora                    66.1640 (1.48)     390.8830 (1.91)      69.9389 (1.42)     12.3158 (1.0)       67.8360 (1.48)     1.7008 (2.15)      207;365       14.2982 (0.71)       7163           1
test_fastavro_with_schema     123.0540 (2.75)     397.6430 (1.95)     136.2530 (2.76)     19.9438 (1.62)     128.8250 (2.82)     8.2955 (10.47)    639;1109        7.3393 (0.36)       5609           1
test_avrora_with_schema       170.7030 (3.82)     457.7960 (2.24)     180.3059 (3.65)     19.9996 (1.62)     177.1220 (3.87)     4.2680 (5.39)      210;292        5.5461 (0.27)       4397           1
test_avro                     260.9860 (5.84)     647.6290 (3.17)     275.1037 (5.57)     30.9256 (2.51)     266.4750 (5.83)     5.8060 (7.33)      194;263        3.6350 (0.18)       2566           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
```

## License

* [MIT LICENSE](LICENSE)

## Contribution

[Contribution guidelines for this project](CONTRIBUTING.md)
