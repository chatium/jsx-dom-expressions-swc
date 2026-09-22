import { createComponent as _$createComponent } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
const template18 = _$createComponent(Comp, {
  get children() {
    return _$memo(() => !!state.dynamic)() ? _$createComponent(Comp, {}) : _$createComponent(Comp, {});
  }
});
