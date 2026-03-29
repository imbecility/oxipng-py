use pyo3::prelude::*;
use pyo3::types::PyBytes;
use indexmap::IndexSet;
use oxipng::{optimize_from_memory as oxipng_optimize_from_memory, Options, Deflater, ZopfliOptions, StripChunks};

#[pyclass(name = "Deflaters")]
#[derive(Clone)]
pub struct PyDeflater {
    inner: Deflater,
}

#[pymethods]
impl PyDeflater {
    #[staticmethod]
    fn libdeflater(compression: u8) -> Self {
        PyDeflater {
            inner: Deflater::Libdeflater { compression },
        }
    }

    #[staticmethod]
    fn zopfli(iterations: std::num::NonZeroU64) -> Self {
        let mut opts = ZopfliOptions::default();
        opts.iteration_count = iterations;
        PyDeflater {
            inner: Deflater::Zopfli(opts),
        }
    }
}

#[pyclass(name = "StripChunks")]
#[derive(Clone)]
pub struct PyStripChunks {
    inner: StripChunks,
}

#[pymethods]
impl PyStripChunks {
    #[staticmethod]
    fn none() -> Self {
        PyStripChunks { inner: StripChunks::None }
    }

    #[staticmethod]
    fn safe() -> Self {
        PyStripChunks { inner: StripChunks::Safe }
    }

    #[staticmethod]
    fn all() -> Self {
        PyStripChunks { inner: StripChunks::All }
    }

    #[staticmethod]
    fn strip(val: Vec<Vec<u8>>) -> PyResult<Self> {
        let mut set = IndexSet::new();
        for v in val {
            if v.len() == 4 {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&v);
                set.insert(arr);
            } else {
                return Err(pyo3::exceptions::PyValueError::new_err("The chunk name must be exactly 4 bytes (for example: b'iTXt')"));
            }
        }
        Ok(PyStripChunks { inner: StripChunks::Strip(set) })
    }

    #[staticmethod]
    fn keep(val: Vec<Vec<u8>>) -> PyResult<Self> {
        let mut set = IndexSet::new();
        for v in val {
            if v.len() == 4 {
                let mut arr =[0u8; 4];
                arr.copy_from_slice(&v);
                set.insert(arr);
            } else {
                return Err(pyo3::exceptions::PyValueError::new_err("The chunk name must be exactly 4 bytes (for example: b'iTXt')"));
            }
        }
        Ok(PyStripChunks { inner: StripChunks::Keep(set) })
    }
}

#[pyfunction]
#[pyo3(signature = (
    data,
    level=2,
    fix_errors=false,
    force=false,
    interlace=None,
    optimize_alpha=false,
    bit_depth_reduction=true,
    color_type_reduction=true,
    palette_reduction=true,
    grayscale_reduction=true,
    idat_recoding=true,
    scale_16=false,
    fast_evaluation=false,
    deflate=None,
    strip=None
))]
#[allow(clippy::too_many_arguments)]
fn optimize_from_memory(
    py: Python<'_>,
    data: &[u8],
    level: u8,
    fix_errors: bool,
    force: bool,
    interlace: Option<bool>,
    optimize_alpha: bool,
    bit_depth_reduction: bool,
    color_type_reduction: bool,
    palette_reduction: bool,
    grayscale_reduction: bool,
    idat_recoding: bool,
    scale_16: bool,
    fast_evaluation: bool,
    deflate: Option<PyDeflater>,
    strip: Option<PyStripChunks>,
) -> PyResult<Py<PyBytes>> {

    let mut options = Options::from_preset(level);

    options.fix_errors = fix_errors;
    options.force = force;
    options.interlace = interlace;
    options.optimize_alpha = optimize_alpha;
    options.bit_depth_reduction = bit_depth_reduction;
    options.color_type_reduction = color_type_reduction;
    options.palette_reduction = palette_reduction;
    options.grayscale_reduction = grayscale_reduction;
    options.idat_recoding = idat_recoding;
    options.scale_16 = scale_16;
    options.fast_evaluation = fast_evaluation;

    if let Some(d) = deflate {
        options.deflater = d.inner;
    }

    if let Some(s) = strip {
        options.strip = s.inner;
    }

    match oxipng_optimize_from_memory(data, &options) {
        Ok(optimized_data) => Ok(PyBytes::new(py, &optimized_data).into()),
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
    }
}

#[pymodule]
fn oxipng_py(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDeflater>()?;
    m.add_class::<PyStripChunks>()?;
    m.add_function(wrap_pyfunction!(optimize_from_memory, m)?)?;
    Ok(())
}
