# gpui-fast retained accessibility hierarchy repair

The initial commit imports the published `gpui-fast` 0.1.13 crate unchanged, except for Cargo's local
download markers. Its archive SHA-256 is
`7ee67914f779fdcfce4c7f59f6ba8fa68f680bd137648b25a801cd5d9cb1b8ab`.

The source delta applies upstream commits
`2d526e0a24f456832818fae81020bdfa1e8da7d1` (longbridge/gpui-fast#122) and
`97439ecf580765469aff56c534c5d1d6af336328` (#126), including the upstream regression test.
The registry package's paths omit `crates/gpui/`; the patches otherwise apply unchanged.

Synthetic accessibility children can describe a nested hierarchy. Retained replay must preserve that
hierarchy when the synthetic builder replaces the parent's children, rather than flatten every recorded
leaf under the parent and give a child two parents. The malformed tree aborts AccessKit's consumer.
See https://github.com/hesong12/kiweely/issues/6359.

The dependency graph, rendering, platform crates and public API are unchanged. Kiweely's separately pinned
macOS frame-pacing and Windows child-window repairs continue to use the same published core version.
