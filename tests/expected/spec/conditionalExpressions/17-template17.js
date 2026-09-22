import { createComponent as _$createComponent } from "r-dom";
import { memo as _$memo } from "r-dom";
const template17 = _$createComponent(Comp, {
  get render() {
    return _$memo(() => !!state.dynamic)() ? _$createComponent(Comp, {}) : _$createComponent(Comp, {});
  }
});
