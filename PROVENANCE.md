# gpui-fast-windows snap-layout patch

Commit `a95182e1ae85e275dc6dbff4f18430c4b2440d25` imports the published `gpui-fast-windows` 0.1.17 crate unchanged
from crates.io (archive checksum
`59fb3d59d263955c73b7acf8b19df9ade3ef2805eae81ef740a91217b38e04e2`). The normalized manifest and registry
dependencies are unchanged, so this single platform crate shares the consumer's gpui-fast 0.1.17 core.

The second commit is the only source delta: a window with `appears_transparent` (the app draws its own title bar)
is created with `WS_CAPTION`. The existing `WM_NCCALCSIZE` handling already keeps the client area over the caption,
so nothing extra is drawn; with the style, Windows 11 opens its snap layouts when the pointer hovers a button the
app marks `WindowControlArea::Max` (verified on Windows 11 build 26200: snap flyout shows with it, does not without).

(Note: the earlier non-topmost DirectComposition `CreateTargetForHwnd(hwnd, false)` patch from 0.1.13 is already
upstream in 0.1.17.)
