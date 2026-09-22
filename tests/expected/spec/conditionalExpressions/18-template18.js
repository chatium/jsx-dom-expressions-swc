import { createComponent as _$createComponent } from "r-dom";
import { memo as _$memo } from "r-dom";
const template18 = _$createComponent(Comp, {
  get children() {
    return _$memo(() => !!state.dynamic)() ? _$createComponent(Comp, {}) : _$createComponent(Comp, {});
  }
});
