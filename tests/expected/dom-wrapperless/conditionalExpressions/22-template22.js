import { createComponent as _$createComponent } from "r-dom";
const template22 = _$createComponent(Comp, {
  get children() {
    return state?.dynamic ? "a" : "b";
  }
});
