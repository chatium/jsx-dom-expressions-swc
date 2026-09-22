import { createComponent as _$createComponent } from "r-dom";
const template12 = _$createComponent(Comp, {
  get render() {
    return state.dynamic ? good() : bad;
  }
});
