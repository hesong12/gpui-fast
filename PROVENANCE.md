# gpui-fast-macos native-demand patch

The parent commit imports the published `gpui-fast-macos` 0.1.13 crate unchanged
from crates.io (archive checksum
`d809f618f836aff7fc654c00c3f6e64d41abd83713140998dd83e8ad6906b1e9`).
The normalized Cargo manifest and registry dependencies remain unchanged so this
single platform crate shares the consumer's existing gpui-fast 0.1.13 core.

The source delta is the macOS native-demand fix from upstream proposal
https://github.com/longbridge/gpui-fast/pull/137, source commit
`3d9fa28` in `hesong12/gpui-fast` (`fix/macos-idle-redraw`).
No other platform, renderer, layout or core behavior is changed.

Consumer failure and real-app evidence:
https://github.com/hesong12/kiweely/issues/5152
