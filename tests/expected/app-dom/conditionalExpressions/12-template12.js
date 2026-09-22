import { createComponent as _$createComponent } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
const template12 = _$createComponent(Comp, {
  get render() {
    return _$memo(() => !!state.dynamic)() ? good() : bad;
  }
});
