import { createComponent as _$createComponent } from "r-dom";
import { memo as _$memo } from "r-dom";
const template14 = _$createComponent(Comp, {
  get render() {
    return _$memo(() => !!state.dynamic)() && good();
  }
});
