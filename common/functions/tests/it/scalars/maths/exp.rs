// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datavalues::prelude::*;
use common_exception::Result;
use common_functions::scalars::*;

#[test]
fn test_exp_function() -> Result<()> {
    struct Test {
        name: &'static str,
        display: &'static str,
        columns: DataColumn,
        expect: DataColumn,
        error: &'static str,
    }
    let tests = vec![
        Test {
            name: "exp-with-literal",
            display: "EXP",
            columns: Series::new(vec![2]).into(),
            expect: Series::new(vec![7.38905609893065_f64]).into(),
            error: "",
        },
        Test {
            name: "exp-with-series",
            display: "EXP",
            columns: Series::new(vec![2, -2, 0]).into(),
            expect: Series::new(vec![7.38905609893065_f64, 0.1353352832366127, 1_f64]).into(),
            error: "",
        },
    ];
    let func = ExpFunction::try_create("exp")?;
    for t in tests {
        let rows = t.columns.len();

        let columns = vec![DataColumnWithField::new(
            t.columns.clone(),
            DataField::new("dummpy", t.columns.data_type(), false),
        )];

        if let Err(e) = func.eval(&columns, rows) {
            assert_eq!(t.error, e.to_string(), "{}", t.name);
        }

        // Display check.
        let expect_display = t.display.to_string();
        let actual_display = format!("{}", func);
        assert_eq!(expect_display, actual_display);

        let v = &(func.eval(&columns, rows)?);
        assert_eq!(v, &t.expect, "{}", t.name);

        let return_type = v.data_type();
        let expected_type = func.return_type(&[t.columns.data_type()])?;
        assert_eq!(expected_type, return_type);
    }
    Ok(())
}
