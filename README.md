# oxipng-py

**oxipng-py** is a fast and efficient Python wrapper for the multi-threaded PNG optimizer [oxipng](https://github.com/oxipng/oxipng), written in Rust.

`oxipng-py` processes images **entirely in memory**. It accepts and returns Python `bytes`, completely bypassing disk I/O operations, making it blazingly fast and ideal for web servers, data pipelines, and serverless functions.

It is built using [PyO3](https://github.com/PyO3/pyo3) and [Maturin](https://github.com/PyO3/maturin).

## Features
- 🚀 **In-memory processing**: No temporary files required.
- 🦀 **Powered by Rust**: Uses the exact same underlying engine as the `oxipng` CLI.
- 🔄 **Always up to date**: Compiled directly from the upstream `oxipng` repository.
- 🎛️ **Advanced options**: Full control over optimization levels, metadata stripping, interlacing, and compression algorithms (Zopfli / libdeflater).
- 📦 **Cross-platform**: Pre-built wheels for Windows, Linux (including musl), and macOS.

## Installation

```bash
uv add oxipng-py
```

or

```bash
pip install oxipng-py
```

## Usage

### Basic Optimization

The simplest way to use the library is to pass your image bytes and specify an optimization level (0-6):

```python
import oxipng_py

# Read original PNG
with open("orig.png", "rb") as f:
    orig_data = f.read()

# Optimize in memory
optimized_data = oxipng_py.optimize_from_memory(orig_data, level=2)

# Save the result
with open("min.png", "wb") as f:
    f.write(optimized_data)
```

### Advanced Optimization (Max Compression)

You can pass multiple parameters to achieve maximum compression, such as removing all metadata (EXIF, ICC profiles) and using the highest optimization level:

```python
import oxipng_py

with open("orig.png", "rb") as f:
    orig_data = f.read()

# Equivalent to CLI: oxipng -o 6 --strip all --scale16
optimized_data = oxipng_py.optimize_from_memory(
    orig_data,
    level=6,
    scale_16=True,
    strip=oxipng_py.StripChunks.all()
)

with open("py_min.png", "wb") as f:
    f.write(optimized_data)
```

### Managing Metadata (StripChunks)

You have fine-grained control over which metadata chunks to keep or remove using `oxipng_py.StripChunks`:

- `StripChunks.none()`: Keep all metadata.
- `StripChunks.safe()`: (Default) Strip metadata that is not required for rendering, except for ICC profiles.
- `StripChunks.all()`: Strip all non-critical metadata.
- `StripChunks.strip([b"iTXt", b"tEXt"])`: Strip specific chunks.
- `StripChunks.keep([b"iCCP"])`: Strip all metadata except the specified ones.

### Deflater Selection

You can also specify the compression algorithm (libdeflater or Zopfli):

```python
import oxipng_py

# Using Zopfli for extreme compression (slower, but smaller file size)
optimized_data = oxipng_py.optimize_from_memory(
    orig_data,
    level=6,
    deflate=oxipng_py.Deflaters.zopfli(iterations=15)
)
```

## API Reference

```python
def optimize_from_memory(
        data: bytes,
        level: int = 6,
        fix_errors: bool = False,
        force: bool = False,
        interlace: bool = False,
        optimize_alpha: bool = False,
        bit_depth_reduction: bool = True,
        color_type_reduction: bool = True,
        palette_reduction: bool = True,
        grayscale_reduction: bool = True,
        idat_recoding: bool = True,
        scale_16: bool = False,
        fast_evaluation: bool = False,
        deflate: Deflaters | None = None,
        strip: StripChunks | None = StripChunks.all(),
) -> bytes:
    """
    Optimizes PNG image data provided as bytes.

    This function provides a programmatic interface to the features of the oxipng utility,
    allowing for flexible configuration of image compression and transformation parameters.

    Note:
        The `optimize_alpha` and `scale_16` options are lossy operations, although
        visually imperceptible. Their use may be unacceptable for certain applications.

    Args:
        data (bytes): Input PNG data as a byte string.
        level (int, optional): Optimization level from 0 (fastest) to 6 (maximum compression).
            Defaults to 6.
        fix_errors (bool, optional): Fixes errors in the PNG structure, such as invalid
            checksums. Defaults to False.
        force (bool, optional): Writes the result even if it is larger than the original
            file. Defaults to False.
        interlace (bool, optional): Applies Adam7 interlacing. Defaults to False.
        optimize_alpha (bool, optional): Performs additional alpha channel optimization
            by modifying the color values of fully transparent pixels. Defaults to False.
        bit_depth_reduction (bool, optional): Allows reduction of bit depth if possible
            without loss of information. Defaults to True.
        color_type_reduction (bool, optional): Allows changing the color type (e.g.,
            from RGB to indexed). Defaults to True.
        palette_reduction (bool, optional): Allows removal of unused entries from
            the palette. Defaults to True.
        grayscale_reduction (bool, optional): Allows conversion to grayscale if the
            image contains no color. Defaults to True.
        idat_recoding (bool, optional): Allows re-compression of IDAT data blocks.
            Defaults to True.
        scale_16 (bool, optional): Forcibly reduces 16-bit channels to 8-bit
            (lossy operation). Defaults to False.
        fast_evaluation (bool, optional): Performs a quick evaluation of filters to
            select the best one before main compression. Defaults to False.
        deflate (Deflaters | None, optional): Specifies the deflate compressor to use
            (e.g., libdeflate or zopfli). Default depends on the optimization level.
        strip (StripChunks | None, optional): Determines which metadata (chunks)
            to remove. Possible values: 'safe', 'all', or a custom list.
            Defaults to StripChunks.all().

    Returns:
        bytes: Optimized PNG data as a byte string.

    Raises:
        PngError: Occurs if the PNG data is invalid and cannot be processed.
    """
    ...
```

## License

This project is licensed under the same terms as the original `oxipng` project (MIT License).
