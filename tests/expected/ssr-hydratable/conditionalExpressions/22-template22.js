import { createComponent as _$createComponent } from "r-server";
const template22 = _$createComponent(Comp, {
  get children() {
    return state?.dynamic ? "a" : "b";
  }
});
