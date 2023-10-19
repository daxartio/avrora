use apache_avro::Reader;
use apache_avro::Schema;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
struct Avro {
    schema: Schema,
}

#[pymethods]
impl Avro {
    #[new]
    fn parse_schema(raw_schema: &str) -> PyResult<Self> {
        let Ok(schema) = Schema::parse_str(raw_schema) else {
            return Err(PyValueError::new_err("Invalid schema"));
        };
        Ok(Avro { schema })
    }

    pub fn parse(&self, input: &[u8]) -> PyResult<String> {
        let reader = Reader::with_schema(&self.schema, &input[..]).unwrap();
        for record in reader {
            return Ok(format!("{:?}", record.unwrap()).to_string());
        }
        Err(PyValueError::new_err("Empty reader"))
    }
}

#[pyfunction]
fn parse(input: &[u8]) -> PyResult<String> {
    let reader = Reader::new(&input[..]).unwrap();
    for record in reader {
        return Ok(format!("{:?}", record.unwrap()));
    }
    Err(PyValueError::new_err("Empty reader"))
}

/// avrora package.
#[pymodule]
fn _avrora(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Avro>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
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
