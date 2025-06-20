use apache_avro::types::Value;
use apache_avro::Reader;
use apache_avro::Schema;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDelta;
use pyo3::types::PyDict;
use pyo3::types::PyList;
use pyo3::types::PyType;
use pyo3::BoundObject;

fn to_pyobject(py: Python, datum: Value) -> PyResult<PyObject> {
    match datum {
        Value::Null => Ok(py.None()),
        Value::Boolean(b) => Ok(b.into_pyobject(py).unwrap().into_any().unbind()),
        Value::Int(n) => Ok(n.into_pyobject(py).unwrap().into_any().unbind()),
        Value::Long(n) => Ok(n.into_pyobject(py).unwrap().into_any().unbind()),
        Value::Float(x) => Ok(x.into_pyobject(py).unwrap().into_any().unbind()),
        Value::Double(x) => Ok(x.into_pyobject(py).unwrap().into_any().unbind()),
        Value::Bytes(bytes) => Ok(bytes.into_pyobject(py).unwrap().into_any().unbind()),
        Value::String(string) => Ok(string.into_pyobject(py).unwrap().into_any().unbind()),
        Value::Fixed(_, bytes) => Ok(bytes.into_pyobject(py).unwrap().into_any().unbind()),
        Value::Enum(_, symbol) => Ok(symbol.into_pyobject(py).unwrap().into_any().unbind()),
        Value::Union(_, item) => to_pyobject(py, *item),
        Value::Array(items) => {
            let list = PyList::empty(py);
            for item in items {
                list.append(to_pyobject(py, item)?)?;
            }
            Ok(list.into_pyobject(py).unwrap().into_any().unbind())
        }
        Value::Map(items) => {
            let dict = PyDict::new(py);
            for (key, value) in items {
                dict.set_item(key, to_pyobject(py, value)?)?;
            }
            Ok(dict.into_pyobject(py).unwrap().into_any().unbind())
        }
        Value::Record(fields) => {
            let dict = PyDict::new(py);
            for (name, value) in fields {
                dict.set_item(name, to_pyobject(py, value)?)?;
            }
            Ok(dict.into_pyobject(py).unwrap().into_any().unbind())
        }
        Value::Date(date) => Ok(date.into_pyobject(py).unwrap().into_any().unbind()),
        Value::TimeMillis(n) => Ok(n.into_pyobject(py).unwrap().into_any().unbind()),
        Value::TimeMicros(n) => Ok(n.into_pyobject(py).unwrap().into_any().unbind()),
        Value::TimestampMillis(n) => Ok(n.into_pyobject(py).unwrap().into_any().unbind()),
        Value::TimestampMicros(n) => Ok(n.into_pyobject(py).unwrap().into_any().unbind()),
        Value::LocalTimestampMillis(n) => Ok(n.into_pyobject(py).unwrap().into_any().unbind()),
        Value::LocalTimestampMicros(n) => Ok(n.into_pyobject(py).unwrap().into_any().unbind()),
        Value::Decimal(_) => todo!(),
        Value::Duration(_) => {
            let delta = PyDelta::new(py, 0, 0, 0, false)?;
            Ok(delta.into_pyobject(py).unwrap().into_any().unbind())
        }
        Value::Uuid(u) => Ok(u.as_bytes().into_pyobject(py).unwrap().into_any().unbind()),
        Value::BigDecimal(_big_decimal) => todo!(),
        Value::TimestampNanos(_) => todo!(),
        Value::LocalTimestampNanos(_) => todo!(),
    }
}

#[pyclass]
struct Avro {
    schema: Option<Schema>,
}

#[pymethods]
impl Avro {
    #[new]
    fn new() -> Self {
        Avro { schema: None }
    }

    #[classmethod]
    fn with_schema(_cls: &Bound<'_, PyType>, raw_schema: &str) -> PyResult<Self> {
        let Ok(schema) = Schema::parse_str(raw_schema) else {
            return Err(PyValueError::new_err("Invalid schema"));
        };
        Ok(Avro {
            schema: Some(schema),
        })
    }

    pub fn parse(&self, py: Python, input: &[u8]) -> PyResult<Vec<PyObject>> {
        let reader = if let Some(ref schema) = self.schema {
            Reader::with_schema(schema, input)
        } else {
            Reader::new(input)
        };

        if let Err(err) = reader {
            Err(PyValueError::new_err(format!("Error: {}", err)))
        } else {
            let res: Vec<PyObject> = reader
                .unwrap()
                .map(|read| to_pyobject(py, read.unwrap()).unwrap())
                .collect();
            Ok(res)
        }
    }
}

/// avrora package.
#[pymodule]
fn _avrora(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Avro>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use apache_avro::Reader;
    use apache_avro::Schema;

    #[test]
    fn test() {
        let reader_raw_schema = r#"
            {
                "type": "record",
                "name": "test",
                "fields": [
                    {"name": "a", "type": "long", "default": 42},
                    {"name": "b", "type": "string"}
                ]
            }
        "#;

        let schema = Schema::parse_str(reader_raw_schema).unwrap();
        let input = [
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
        ];
        let reader = Reader::with_schema(&schema, &input[..]).unwrap();
        for record in reader {
            println!("{:?}", format!("{:?}", record.unwrap()));
        }
    }
}
