# Reactive

## Source

name: @nexus/reactive (internal)
branched from version: 0.1.0

## Changes

1. Added custom interface through `.x` property

```ts
interface SuperDiv {
	show(): void,
	hide(): void,
}

const div = Div<SuperDiv>();
div.x.show = () => { };
div.x.hide = () => { };

div.x.show();
```
