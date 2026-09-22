import { createComponent as _$createComponent } from "r-dom";
import { memo as _$memo } from "r-dom";
const template26 = _$createComponent(Comp, {
  get children() {
    return state.dynamic ?? _$createComponent(Comp, {});
  }
});
