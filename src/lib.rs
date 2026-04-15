/*
 * // Copyright 2026 zhaoxi826
 * //
 * // Licensed under the Apache License, Version 2.0 (the "License");
 * // you may not use this file except in compliance with the License.
 * // You may obtain a copy of the License at
 * //
 * //     http://www.apache.org/licenses/LICENSE-2.0
 * //
 * // Unless required by applicable law or agreed to in writing, software
 * // distributed under the License is distributed on an "AS IS" BASIS,
 * // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * // See the License for the specific language governing permissions and
 * // limitations under the License.
 */

pub mod installer;
pub mod manifest;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
#[pyo3(signature = (url, path=String::new(), cache=String::from(".cache"), output=None))]
fn install_skill(url: String, path: String, cache: String, output: Option<String>) -> PyResult<String> {
    let skill = manifest::skill::model::SkillModel::install(url, cache, path, output);
    skill.analysis().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Analysis failed: {}", e))
    })?;
    Ok(skill.skill_path)
}

#[pymodule]
fn viceroy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(install_skill, m)?)?;
    Ok(())
}