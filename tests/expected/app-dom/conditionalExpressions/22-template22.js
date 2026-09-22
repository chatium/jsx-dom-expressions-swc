import { createComponent as _$createComponent } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
const template22 = _$createComponent(Comp, {
  get children() {
    return state?.dynamic ? "a" : "b";
  }
});
