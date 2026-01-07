// <https://github.com/KallDrexx/rust-media-libs>
// License: See ../LICENSE-APACHE and ../LICENSE-MIT
// Modifications Copyright 2026 Tobias Bessler

use quick_byte::QuickRead;

use super::{Amf0Value, error::DeserializationError, markers};
use std::collections::HashMap;
use std::io::Read;

struct ObjectProperty {
    label: String,
    value: Amf0Value,
}

// Turns any readable byte stream and converts it into an array of AMF0 values
pub fn deserialize<R: Read>(bytes: &mut R) -> Result<Vec<Amf0Value>, DeserializationError> {
    let mut references = vec![];
    let mut results = vec![];

    loop {
        match read_next_value(bytes, &mut references)? {
            Some(x) => results.push(x),
            None => break,
        };
    }

    Ok(results)
}

fn read_next_value<R: Read>(
    bytes: &mut R,
    references: &mut Vec<Amf0Value>,
) -> Result<Option<Amf0Value>, DeserializationError> {
    let mut buffer: [u8; 1] = [0];
    let bytes_read = bytes.read(&mut buffer)?;

    if bytes_read == 0 {
        return Ok(None);
    }

    if buffer[0] == markers::OBJECT_END_MARKER {
        return Ok(None);
    }

    match buffer[0] {
        markers::BOOLEAN_MARKER => parse_bool(bytes).map(Some),
        markers::NULL_MARKER => parse_null().map(Some),
        markers::UNDEFINED_MARKER => parse_undefined().map(Some),
        markers::NUMBER_MARKER => parse_number(bytes).map(Some),
        markers::OBJECT_MARKER => parse_object(bytes, references).map(Some),
        markers::ECMA_ARRAY_MARKER => parse_ecma_array(bytes, references).map(Some),
        markers::STRING_MARKER => parse_string(bytes).map(Some),
        markers::STRICT_ARRAY_MARKER => parse_strict_array(bytes, references).map(Some),
        markers::REFERENCE_MARKER => parse_reference(bytes, references).map(Some),
        _ => Err(DeserializationError::UnknownMarker { marker: buffer[0] }),
    }
}

fn parse_number<R: Read>(bytes: &mut R) -> Result<Amf0Value, DeserializationError> {
    let number = bytes.read_f64_be()?;
    let value = Amf0Value::Number(number);

    Ok(value)
}

fn parse_null() -> Result<Amf0Value, DeserializationError> {
    Ok(Amf0Value::Null)
}

fn parse_undefined() -> Result<Amf0Value, DeserializationError> {
    Ok(Amf0Value::Undefined)
}

fn parse_bool<R: Read>(bytes: &mut R) -> Result<Amf0Value, DeserializationError> {
    let value = bytes.read_u8()?;

    if value == 1 {
        Ok(Amf0Value::Boolean(true))
    } else {
        Ok(Amf0Value::Boolean(false))
    }
}

fn parse_string<R: Read>(bytes: &mut R) -> Result<Amf0Value, DeserializationError> {
    let length = bytes.read_u16_be()?;
    let mut buffer: Vec<u8> = vec![0_u8; length as usize];
    bytes.read_exact(&mut buffer)?;

    let value = String::from_utf8(buffer)?;
    Ok(Amf0Value::Utf8String(value))
}

fn parse_object<R: Read>(
    bytes: &mut R,
    references: &mut Vec<Amf0Value>,
) -> Result<Amf0Value, DeserializationError> {
    let mut properties = HashMap::new();

    loop {
        match parse_object_property(bytes, references)? {
            Some(property) => properties.insert(property.label, property.value),
            None => break,
        };
    }

    let deserialized_value = Amf0Value::Object(properties);
    references.push(deserialized_value.clone());
    Ok(deserialized_value)
}

fn parse_ecma_array<R: Read>(
    bytes: &mut R,
    references: &mut Vec<Amf0Value>,
) -> Result<Amf0Value, DeserializationError> {
    // An ECMA array is an array of values indexed via strings instead of numeric indexes (so
    // essentially a hash map).  It seems functionally equivalent to an object so for simplicity
    // treat it as such.

    // While the spec says it gives you the count of items in the array, it is vague about if
    // the object end marker is used.  In real world usages I have found the associative array
    // actually ends with a 0x000009 ending (same as objects do).  If we don't consume this
    // then the buffer will start at that ending and funky things will happen.  So for now it seems
    // like we can ignore the associative count and just read exactly as we would an object.

    let _associative_count = bytes.read_u32_be()?;
    parse_object(bytes, references)
}

fn parse_strict_array<R: Read>(
    bytes: &mut R,
    references: &mut Vec<Amf0Value>,
) -> Result<Amf0Value, DeserializationError> {
    let _array_count = bytes.read_u32_be()?;
    let mut values: Vec<Amf0Value> = Vec::new();

    for _ in 0.._array_count {
        match read_next_value(bytes, references)? {
            Some(value) => {
                values.push(value);
            }
            None => break,
        };
    }

    references.push(Amf0Value::StrictArray(values.clone()));

    Ok(Amf0Value::StrictArray(values))
}

fn parse_object_property<R: Read>(
    bytes: &mut R,
    references: &mut Vec<Amf0Value>,
) -> Result<Option<ObjectProperty>, DeserializationError> {
    let label_length = bytes.read_u16_be()?;
    if label_length == 0 {
        // Next byte should be the end of object marker.  We need to read this
        // to make sure we progress the current position.
        let byte = bytes.read_u8()?;
        if byte != markers::OBJECT_END_MARKER {
            return Err(DeserializationError::UnexpectedEmptyObjectPropertyName);
        }

        return Ok(None);
    }

    let mut label_buffer = vec![0; label_length as usize];
    bytes.read_exact(&mut label_buffer)?;

    let label = String::from_utf8(label_buffer)?;

    match read_next_value(bytes, references)? {
        None => Err(DeserializationError::UnexpectedEof),
        Some(property_value) => Ok(Some(ObjectProperty {
            label,
            value: property_value,
        })),
    }
}

fn parse_reference<R: Read>(
    bytes: &mut R,
    references: &Vec<Amf0Value>,
) -> Result<Amf0Value, DeserializationError> {
    let index = bytes.read_u16_be()?;
    Ok(references[index as usize].clone())
}

#[cfg(test)]
mod tests {
    use quick_byte::QuickWrite;

    use super::super::Amf0Value;
    use super::deserialize;
    use super::markers;
    use std::collections::HashMap;
    use std::io::Cursor;

    #[test]
    fn can_deserialize_strict_array() {
        let mut vector = vec![];
        vector.push(markers::STRICT_ARRAY_MARKER);
        vector.write_u32_be(2).unwrap();
        vector.push(markers::NUMBER_MARKER);
        vector.write_f64_be(1.0).unwrap();
        vector.push(markers::NUMBER_MARKER);
        vector.write_f64_be(2.0).unwrap();

        let mut input = Cursor::new(vector);
        let result = deserialize(&mut input).unwrap();

        let array = vec![Amf0Value::Number(1.0), Amf0Value::Number(2.0)];
        let expected = vec![Amf0Value::StrictArray(array)];
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_number() {
        let number: f64 = 332.0;

        let mut vector = vec![];
        vector.write_u8(markers::NUMBER_MARKER).unwrap();
        vector.write_f64_be(number).unwrap();

        let mut input = Cursor::new(vector);
        let result = deserialize(&mut input).unwrap();

        let expected = vec![Amf0Value::Number(number)];
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_true_boolean() {
        let mut vector = vec![];
        vector.write_u8(markers::BOOLEAN_MARKER).unwrap();
        vector.write_u8(1).unwrap();

        let mut input = Cursor::new(vector);
        let result = deserialize(&mut input).unwrap();

        let expected = vec![Amf0Value::Boolean(true)];
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_false_boolean() {
        let mut vector = vec![];
        vector.write_u8(markers::BOOLEAN_MARKER).unwrap();
        vector.write_u8(0).unwrap();

        let mut input = Cursor::new(vector);
        let result = deserialize(&mut input).unwrap();

        let expected = vec![Amf0Value::Boolean(false)];
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_string() {
        let value = "test";

        let mut vector = vec![];
        vector.write_u8(markers::STRING_MARKER).unwrap();
        vector.write_u16_be(value.len() as u16).unwrap();
        vector.extend(value.as_bytes());

        let mut input = Cursor::new(vector);
        let result = deserialize(&mut input).unwrap();

        let expected = vec![Amf0Value::Utf8String(value.to_string())];
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_null() {
        let mut vector = vec![];
        vector.write_u8(markers::NULL_MARKER).unwrap();

        let mut input = Cursor::new(vector);
        let result = deserialize(&mut input).unwrap();

        let expected = vec![Amf0Value::Null];
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_object() {
        const NUMBER: f64 = 332.0;

        let mut vector = vec![];
        vector.push(markers::OBJECT_MARKER);
        vector.write_u16_be(4).unwrap();
        vector.extend("test".as_bytes());
        vector.push(markers::NUMBER_MARKER);
        vector.write_f64_be(NUMBER).unwrap();
        vector.write_u16_be(markers::UTF_8_EMPTY_MARKER).unwrap();
        vector.push(markers::OBJECT_END_MARKER);

        let mut input = Cursor::new(vector);
        let result = deserialize(&mut input).unwrap();

        let mut properties = HashMap::new();
        properties.insert("test".to_string(), Amf0Value::Number(NUMBER));

        let expected = vec![Amf0Value::Object(properties)];
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_emca_array() {
        let mut vector = vec![];
        vector.push(markers::ECMA_ARRAY_MARKER);
        vector.write_u32_be(2).unwrap();
        vector.write_u16_be(5).unwrap();
        vector.extend("test1".as_bytes());
        vector.push(markers::NUMBER_MARKER);
        vector.write_f64_be(1.0).unwrap();
        vector.write_u16_be(5).unwrap();
        vector.extend("test2".as_bytes());
        vector.write_u8(markers::STRING_MARKER).unwrap();
        vector.write_u16_be(6).unwrap();
        vector.extend("second".as_bytes());
        vector.write_u16_be(markers::UTF_8_EMPTY_MARKER).unwrap();
        vector.push(markers::OBJECT_END_MARKER);

        let mut input = Cursor::new(vector);
        let result = deserialize(&mut input).unwrap();

        let mut properties = HashMap::new();
        properties.insert("test1".to_string(), Amf0Value::Number(1.0));
        properties.insert(
            "test2".to_string(),
            Amf0Value::Utf8String("second".to_string()),
        );

        let expected = vec![Amf0Value::Object(properties)];
        assert_eq!(result, expected);
    }

    #[test]
    fn can_deserialize_undefined() {
        let mut vector = vec![];
        vector.write_u8(markers::UNDEFINED_MARKER).unwrap();

        let mut input = Cursor::new(vector);
        let result = deserialize(&mut input).unwrap();

        let expected = vec![Amf0Value::Undefined];
        assert_eq!(result, expected);
    }
}
