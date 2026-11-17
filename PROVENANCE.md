# gpui-fast-macos native-demand patch

Commit `49c14223ef45cdce52733a44f5ed013b32c99cd5` imports the published `gpui-fast-macos` 0.1.13 crate unchanged
from crates.io (archive checksum
`d809f618f836aff7fc654c00c3f6e64d41abd83713140998dd83e8ad6906b1e9`).
The normalized Cargo manifest and registry dependencies remain unchanged so this
single platform crate shares the consumer's existing gpui-fast 0.1.13 core.

The source delta is the macOS native-demand fix from upstream proposal
https://github.com/longbridge/gpui-fast/pull/137, source commit
`d890f85` in `hesong12/gpui-fast` (`fix/macos-idle-redraw`).
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
