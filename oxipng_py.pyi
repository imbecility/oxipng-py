class Deflaters:
    """
    class for initializing the compression algorithm (Deflater)
    """

    @staticmethod
    def libdeflater(compression: int) -> "Deflaters": ...

    @staticmethod
    def zopfli(iterations: int) -> "Deflaters": ...


class StripChunks:
    """
    class for initializing metadata removal parameters (strip)
    """

    @staticmethod
    def none() -> "StripChunks": ...

    @staticmethod
    def safe() -> "StripChunks": ...

    @staticmethod
    def all() -> "StripChunks": ...

    @staticmethod
    def strip(val: list[bytes]) -> "StripChunks": ...

    @staticmethod
    def keep(val: list[bytes]) -> "StripChunks": ...


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
