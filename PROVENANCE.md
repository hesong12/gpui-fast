# gpui-fast-macos native-demand patch

Commit `e6ebd1af5f19385eae78e7607ffe47358fb6d4b0` imports the published `gpui-fast-macos` 0.1.17 crate unchanged
from crates.io (archive checksum
`c74b7984a584eebefdf69aec1b5d89d928e2efde862006427b09695425117ad0`).
The normalized Cargo manifest and registry dependencies remain unchanged so this
single platform crate shares the consumer's existing gpui-fast 0.1.17 core.

The source delta is the macOS native-demand fix carried forward from `kiweely-macos-0.1.13-frame-pacing`
(`e80784f87231312580d011a41c6284caf18adf1c`, originally proposed in
https://github.com/longbridge/gpui-fast/pull/137):
A frame request that arrives while no Metal link is attached (inside `displayLayer:`'s synchronous draw,
which detaches the link, or before the window is first seen visible) damages the view as well, through a weak
reference, so it is never dropped: GPUI asks only when a clean window turns dirty, and a dropped request left a
window unpainted with every later update ignored (https://github.com/hesong12/kiweely/issues/5712).
A request that a running Metal link will deliver at the next vsync damages nothing: damaging the view for every
request had AppKit draw each scroll and animation frame through `displayLayer:`, which detaches the link, presents
synchronously with the transaction and builds a new link, so frames left the display's pacing and scrolling
stuttered. Only a paused link, no link, or a request during `displayLayer:` that leaves no link attached damages it.

No other platform, renderer, layout or core behavior is changed.

Consumer failure and real-app evidence:
https://github.com/hesong12/kiweely/issues/5152
