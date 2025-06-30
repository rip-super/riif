# RIIF - Rust Image Interchange Format

**RIIF** is a custom image file format and viewer built in Rust.  
It features a complete lossless compression format and a native image viewer with zoom, pan, and metadata support.

---

## 📦 Features

- 🧪 Custom RIIF image format (`.riif`)
- 🖼️ Image compression + decompression using PNG-style filters and `zlib`
- 🧵 Image viewer built with [`eframe`](https://github.com/emilk/egui)
- 🔍 Smooth zooming (slider & scroll wheel)
- 📦 CLI for converting to and from `.riif`

---

## 🛠️ Usage

```bash
riif --encode image.png        # Compress to RIIF
riif --decode image.riif    # Decompress to PNG
riif --view image.riif        # Launch viewer
```

---

## 🖼️ Viewer
When opening a .riif image, you can:

- Zoom in/out (scroll wheel or slider)
- Reset zoom
- View dimensions and file size
- Pan the image using scroll bars

---

## 📁 File Format (RIIF)
Header: 
- Magic: 4 Bytes (`RIIF`)
- Width: 4 Bytes (LE)
- Height: 4 Bytes (LE)

Filter Data:
- Filter Types: `Height` Bytes long  (`None`, `Sub`, `Up`, etc.)

Filtered RGBA Data:
- Zlib-compressed RGBA data

---

## 🪟 Windows Integration
I have added 2 binaries to this crate, 1 for `riif.exe` and another for `riif_viewer.exe`. The latter is for if you wish to associate the `.riif` extension to a program. When using `riif_viewer.exe` an additional cmd window will not appear, making it perfect for allowing the viewer to open when double-clicking a `.riif` file. However, this functionality may take some additional steps to get working.
