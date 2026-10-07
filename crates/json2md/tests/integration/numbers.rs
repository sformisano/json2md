use std::error::Error as _;

use json2md::{Error, View};
use serde_json::{Number, Value, json};

#[test]
fn numbers_preserve_interpolation_type_tests_and_arithmetic() {
    let view = View::new(
        &json!({
            "type": "object",
            "required": ["value"],
            "properties": {"value": {"type": "number"}}
        }),
        "{{ value }}|{{ value is number }}|{{ value + 1 }}",
    )
    .unwrap();

    for (value, expected) in [
        (json!(42), "42|True|43"),
        (json!(-7), "-7|True|-6"),
        (json!(1.25), "1.25|True|2.25"),
    ] {
        assert_eq!(view.render(&json!({"value": value})).unwrap(), expected);
    }
}

#[test]
fn nested_numbers_preserve_numeric_behavior_and_large_integer_output() {
    let view = View::new(
        &json!(true),
        "{{ record.count + values[0] }}|{{ values[0] is number }}|{{ max_unsigned }}|{{ min_signed }}",
    )
    .unwrap();
    let input = json!({
        "record": {"count": 7},
        "values": [3],
        "max_unsigned": u64::MAX,
        "min_signed": i64::MIN
    });

    assert_eq!(
        view.render(&input).unwrap(),
        "10|True|18446744073709551615|-9223372036854775808"
    );
}

#[test]
fn renders_finite_float_extremes() {
    let view = View::new(&json!(true), "{{ value is number }}|{{ value > 0 }}").unwrap();

    for value in [f64::MAX, f64::MIN_POSITIVE, f64::from_bits(1)] {
        assert_eq!(view.render(&json!({"value": value})).unwrap(), "True|True");
    }
}

#[test]
fn wide_integer_and_exponent_limits_follow_the_json_representation() {
    let view = View::new(&json!(true), "{{ value }}").unwrap();

    if let Some(maximum) = Number::from_u128(u128::MAX) {
        // Feature unification permits JSON integers beyond the default 64-bit representation.
        assert_eq!(
            view.render(&json!({"value": maximum})).unwrap(),
            "340282366920938463463374607431768211455"
        );
        let minimum = Number::from_i128(i128::MIN).expect("128-bit JSON representation");
        assert_eq!(
            view.render(&json!({"value": minimum})).unwrap(),
            "-170141183460469231731687303715884105728"
        );

        for raw in [
            "340282366920938463463374607431768211456",
            "-170141183460469231731687303715884105729",
            "1e999",
        ] {
            let value: Value = serde_json::from_str(raw).expect("arbitrary-precision JSON number");
            let input = json!({"value": value});
            view.validate(&input)
                .expect("the schema accepts the number");
            let error = view
                .render(&input)
                .expect_err("the renderer rejects an out-of-range number");
            assert!(matches!(error, Error::Render { .. }));
            assert!(error.source().is_some());
        }
    } else {
        assert!(Number::from_i128(i128::MIN).is_none());
        assert!(serde_json::from_str::<Value>("1e999").is_err());
        assert_eq!(
            view.render(&json!({"value": u64::MAX})).unwrap(),
            u64::MAX.to_string()
        );
    }
}
