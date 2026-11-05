import png
from pathlib import Path

ROOT = Path(__file__).resolve().parent / 'test-png'

# --- RGB 8-bits ---
rows = [
    [(255,0,0), (0,255,0)],
    [(0,0,255), (255,255,255)],
]
png.Writer(width=2, height=2, bitdepth=8, greyscale=False, alpha=False).write(
    open(ROOT / 'rgb8.png', 'wb'),
    [[c for px in row for c in px] for row in rows]
)

# --- RGBA 16-bits ---
rows = [
    [(65_535,0,0,65_535 // 2), (0,0,0,65_535)],
    [(0,65_535,65_535,65_535), (65_535,65_535,65_535,65_535 // 2)],
]
png.Writer(width=2, height=2, bitdepth=16, greyscale=False, alpha=True).write(
    open(ROOT / 'rgba16.png', 'wb'),
    [[c for px in row for c in px] for row in rows]
)

# --- Grayscale 8-bits ---
rows = [
    [(255,), (200,)],
    [(127,), (0,)],
]
png.Writer(width=2, height=2, bitdepth=8, greyscale=True, alpha=False).write(
    open(ROOT / 'gray8.png', 'wb'),
    [[c for px in row for c in px] for row in rows]
)