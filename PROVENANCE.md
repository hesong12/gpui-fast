# gpui-fast-windows snap-layout patch

The first commit imports the published `gpui-fast-windows` 0.1.13 crate unchanged from crates.io (archive checksum
`088654835f1408e1aaa79afcc316530461a14fa1c4e543d47b1881b6ced63f19`). The normalized manifest and registry
dependencies are unchanged, so this single platform crate shares the consumer's gpui-fast 0.1.13 core.

The second commit is the only source delta: a window with `appears_transparent` (the app draws its own title bar)
is created with `WS_CAPTION`. The existing `WM_NCCALCSIZE` handling already keeps the client area over the caption,
so nothing extra is drawn; with the style, Windows 11 opens its snap layouts when the pointer hovers a button the
app marks `WindowControlArea::Max` (verified on Windows 11 build 26200: snap flyout shows with it, does not without).
